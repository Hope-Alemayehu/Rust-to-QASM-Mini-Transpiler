use crate::gate::Gate;

fn extract_qubits(s: &str) -> Vec<String> {
    s.trim_matches(|c| c == '(' || c == ')')
        .split(',')
        .map(|s| s.trim().to_string())
        .collect()
}

pub fn parse_line(line: &str) -> Option<Gate> {
    let line = line.trim();
    if line.is_empty() || line.starts_with("//") {
        return None;
    }

    // Handle qubit declaration
    if let Some(rest) = line.strip_prefix("let ") {
        if let Some((name, _)) = rest.split_once(" = ") {
            return Some(Gate::Qubit(name.trim().to_string()));
        }
    }

    // Handle gate applications
    if let Some((gate_name, args)) = line.split_once('(') {
        if let Some(args) = args.strip_suffix(';') {
            let args = args.trim_end_matches(')');
            
            match gate_name.trim() {
                "H" => {
                    let qubits = extract_qubits(args);
                    if qubits.len() == 1 {
                        return Some(Gate::H(qubits[0].clone()));
                    }
                }
                "X" => {
                    let qubits = extract_qubits(args);
                    if qubits.len() == 1 {
                        return Some(Gate::X(qubits[0].clone()));
                    }
                }
                "Z" => {
                    let qubits = extract_qubits(args);
                    if qubits.len() == 1 {
                        return Some(Gate::Z(qubits[0].clone()));
                    }
                }
                "CX" => {
                    let qubits = extract_qubits(args);
                    if qubits.len() == 2 {
                        return Some(Gate::CX(qubits[0].clone(), qubits[1].clone()));
                    }
                }
                "CCX" | "ccx" => {
                    let qubits = extract_qubits(args);
                    if qubits.len() == 3 {
                        return Some(Gate::CCX(
                            qubits[0].clone(),
                            qubits[1].clone(),
                            qubits[2].clone(),
                        ));
                    }
                }
                "RZ" | "rz" => {
                    let parts: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
                    if parts.len() == 2 {
                        return Some(Gate::RZ(parts[0].to_string(), parts[1].to_string()));
                    }
                }
                "RX" | "rx" => {
                    let parts: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
                    if parts.len() == 2 {
                        return Some(Gate::RX(parts[0].to_string(), parts[1].to_string()));
                    }
                }
                "RY" | "ry" => {
                    let parts: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
                    if parts.len() == 2 {
                        return Some(Gate::RY(parts[0].to_string(), parts[1].to_string()));
                    }
                }
                "MEASURE" | "measure" => {
                    let qubits = extract_qubits(args);
                    if qubits.len() == 1 {
                        return Some(Gate::Measure(qubits[0].clone()));
                    }
                }
                _ => {}
            }
        }
    }

    None
}

pub fn parse_lines(lines: &[&str]) -> Vec<Gate> {
    lines.iter().filter_map(|line| parse_line(line)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ccx() {
        assert!(matches!(
            parse_line("ccx(q0, q1, q2);"),
            Some(Gate::CCX(a, b, c)) if a == "q0" && b == "q1" && c == "q2"
        ));
        
        // Test case sensitivity
        assert!(matches!(
            parse_line("CCX(q0, q1, q2);"),
            Some(Gate::CCX(a, b, c)) if a == "q0" && b == "q1" && c == "q2"
        ));
    }

    #[test]
    fn test_parse_rotations() {
        // Test RZ
        assert!(matches!(
            parse_line("rz(theta, q0);"),
            Some(Gate::RZ(angle, q)) if angle == "theta" && q == "q0"
        ));

        // Test RX with expression
        assert!(matches!(
            parse_line("rx(pi/2, q1);"),
            Some(Gate::RX(angle, q)) if angle == "pi/2" && q == "q1"
        ));

        // Test RY with float
        assert!(matches!(
            parse_line("ry(3.14, q2);"),
            Some(Gate::RY(angle, q)) if angle == "3.14" && q == "q2"
        ));
    }

    #[test]
    fn test_invalid_syntax() {
        // Missing angle for RZ
        assert!(parse_line("rz(q0);").is_none());
        
        // Missing qubit for RZ
        assert!(parse_line("rz(3.14);").is_none());
        
        // Not enough qubits for CCX
        assert!(parse_line("ccx(q0, q1);").is_none());
        
        // Too many qubits for CCX
        assert!(parse_line("ccx(q0, q1, q2, q3);").is_none());
    }
}
