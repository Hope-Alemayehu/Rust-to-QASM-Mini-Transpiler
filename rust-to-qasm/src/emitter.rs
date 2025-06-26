use crate::gate::Gate;
use std::collections::{HashMap, HashSet};

pub fn emit_qasm(gates: &[Gate]) -> String {
    let mut qset = HashSet::new();

    for gate in gates {
        match gate {
            Gate::Qubit(name) => {
                qset.insert(name.clone());
            }
            Gate::H(name) => {
                qset.insert(name.clone());
            }
            Gate::CX(ctrl, target) => {
                qset.insert(ctrl.clone());
                qset.insert(target.clone());
            }
            Gate::Measure(name) => {
                qset.insert(name.clone());
            }
            Gate::X(name) => {
                qset.insert(name.clone());
            }
            Gate::Z(name) => {
                qset.insert(name.clone());
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

    output.push_str("OPENQASM 2.0;\ninclude \"qelib1.inc\";\n\n");
    output.push_str(&format!("qreg q[{}];\n", num_qubits));
    output.push_str(&format!("creg c[{}];\n\n", num_cregs));

    for gate in gates {
        match gate {
            Gate::Qubit(_) => {}

            Gate::H(name) => {
                let idx = qubit_indices.get(name).unwrap();
                output.push_str(&format!("h q[{}];\n", idx));
            }

            Gate::CX(ctrl, target) => {
                let c_idx = qubit_indices.get(ctrl).unwrap();
                let t_idx = qubit_indices.get(target).unwrap();
                output.push_str(&format!("cx q[{}],q[{}];\n", c_idx, t_idx));
            }

            Gate::Measure(name) => {
                let q_idx = qubit_indices.get(name).unwrap();
                let c_idx = measure_order.get(name).unwrap();
                output.push_str(&format!("measure q[{}] -> c[{}];\n", q_idx, c_idx));
            }

            Gate::X(name) => {
                let idx = qubit_indices.get(name).unwrap();
                output.push_str(&format!("x q[{}];\n", idx));
            }

            Gate::Z(name) => {
                let idx = qubit_indices.get(name).unwrap();
                output.push_str(&format!("z q[{}];\n", idx));
            }
        }
    }

    output
}
