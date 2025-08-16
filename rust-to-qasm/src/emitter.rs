use crate::gate::Gate;
use std::collections::{HashMap, HashSet};

pub fn emit_qasm(gates: &[Gate]) -> String {
    let mut qset = HashSet::new();
    let mut has_ccx = false;
    let mut has_rotations = false;

    // First pass: collect all qubit names and check which gates are used
    for gate in gates {
        match gate {
            Gate::Qubit(name) => {
                qset.insert(name.clone());
            }
            Gate::H(name) | Gate::X(name) | Gate::Z(name) | Gate::Measure(name) => {
                qset.insert(name.clone());
            }
            Gate::CX(ctrl, target) => {
                qset.insert(ctrl.clone());
                qset.insert(target.clone());
            }
            Gate::CCX(ctrl1, ctrl2, target) => {
                qset.insert(ctrl1.clone());
                qset.insert(ctrl2.clone());
                qset.insert(target.clone());
                has_ccx = true;
            }
            Gate::RZ(_, qubit) | Gate::RX(_, qubit) | Gate::RY(_, qubit) => {
                qset.insert(qubit.clone());
                has_rotations = true;
            }
        }
    }

    let mut qubit_indices: HashMap<String, usize> = HashMap::new();
    let mut qubits: Vec<String> = qset.into_iter().collect();
    qubits.sort();

    for (i, q) in qubits.iter().enumerate() {
        qubit_indices.insert(q.clone(), i);
    }

    let num_qubits = qubits.len();
    let num_cregs = gates
        .iter()
        .filter(|g| matches!(g, Gate::Measure(_)))
        .count();

    let mut measure_order = HashMap::new();
    let mut c_idx = 0;

    for gate in gates {
        if let Gate::Measure(name) = gate {
            if !measure_order.contains_key(name) {
                measure_order.insert(name.clone(), c_idx);
                c_idx += 1;
            }
        }
    }

    let mut output = String::new();

    // Add header and include standard gates
    output.push_str("OPENQASM 2.0;\n");
    output.push_str("include \"qelib1.inc\";\n\n");
    
    // Add custom gates if needed
    if has_ccx {
        output.push_str("// Toffoli (CCX) gate is included in qelib1.inc\n");
    }
    
    if has_rotations {
        output.push_str("// Rotation gates (rx, ry, rz) are included in qelib1.inc\n");
    }
    
    output.push_str(&format!("qreg q[{}];\n", num_qubits));
    output.push_str(&format!("creg c[{}];\n\n", num_cregs));

    // Process all gates
    for gate in gates {
        match gate {
            Gate::Qubit(_) => {}

            Gate::H(name) => {
                let idx = qubit_indices.get(name).unwrap();
                output.push_str(&format!("h q[{}];\n", idx));
            }

            Gate::X(name) => {
                let idx = qubit_indices.get(name).unwrap();
                output.push_str(&format!("x q[{}];\n", idx));
            }

            Gate::Z(name) => {
                let idx = qubit_indices.get(name).unwrap();
                output.push_str(&format!("z q[{}];\n", idx));
            }

            Gate::CX(ctrl, target) => {
                let c_idx = qubit_indices.get(ctrl).unwrap();
                let t_idx = qubit_indices.get(target).unwrap();
                output.push_str(&format!("cx q[{}],q[{}];\n", c_idx, t_idx));
            }

            Gate::CCX(ctrl1, ctrl2, target) => {
                let c1_idx = qubit_indices.get(ctrl1).unwrap();
                let c2_idx = qubit_indices.get(ctrl2).unwrap();
                let t_idx = qubit_indices.get(target).unwrap();
                output.push_str(&format!("ccx q[{}],q[{}],q[{}];\n", c1_idx, c2_idx, t_idx));
            }

            Gate::RZ(angle, qubit) => {
                let q_idx = qubit_indices.get(qubit).unwrap();
                output.push_str(&format!("rz({}) q[{}];\n", angle, q_idx));
            }

            Gate::RX(angle, qubit) => {
                let q_idx = qubit_indices.get(qubit).unwrap();
                output.push_str(&format!("rx({}) q[{}];\n", angle, q_idx));
            }

            Gate::RY(angle, qubit) => {
                let q_idx = qubit_indices.get(qubit).unwrap();
                output.push_str(&format!("ry({}) q[{}];\n", angle, q_idx));
            }

            Gate::Measure(name) => {
                let q_idx = qubit_indices.get(name).unwrap();
                let c_idx = measure_order.get(name).unwrap();
                output.push_str(&format!("measure q[{}] -> c[{}];\n", q_idx, c_idx));
            }
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emit_ccx() {
        let gates = vec![
            Gate::Qubit("q0".to_string()),
            Gate::Qubit("q1".to_string()),
            Gate::Qubit("q2".to_string()),
            Gate::CCX("q0".to_string(), "q1".to_string(), "q2".to_string()),
        ];
        
        let output = emit_qasm(&gates);
        assert!(output.contains("ccx q[0],q[1],q[2];"));
    }

    #[test]
    fn test_emit_rotations() {
        let gates = vec![
            Gate::Qubit("q0".to_string()),
            Gate::RZ("theta".to_string(), "q0".to_string()),
            Gate::RX("pi/2".to_string(), "q0".to_string()),
            Gate::RY("3.14".to_string(), "q0".to_string()),
        ];
        
        let output = emit_qasm(&gates);
        assert!(output.contains("rz(theta) q[0];"));
        assert!(output.contains("rx(pi/2) q[0];"));
        assert!(output.contains("ry(3.14) q[0];"));
    }
}
