use crate::gate::Gate;

pub fn parse_line(line: &str) -> Option<Gate> {
    let token: Vec<&str> = line.trim().split_whitespace().collect();

    match token.as_slice() {
        ["qubit", name] => Some(Gate::Qubit(name.to_string())),
        ["H", target] => Some(Gate::H(target.to_string())),
        ["CX", ctrl, target] => Some(Gate::CX(ctrl.to_string(), target.to_string())),
        ["MEASURE", target] => Some(Gate::Measure(target.to_string())),
        ["X", target] => Some(Gate::X(target.to_string())),
        ["Z", target] => Some(Gate::Z(target.to_string())),
        _ => None,
    }
}

pub fn parse_lines(lines: &[&str]) -> Vec<Gate> {
    lines.iter().filter_map(|line| parse_line(line)).collect()
}
