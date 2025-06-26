use rust_to_qasm::{emit_qasm, parse_lines};
use std::fs;

fn main() {
    let input = vec![
        "qubit q0",
        "qubit q1",
        "H q0",
        "CX q0 q1",
        "MEASURE q0",
        "MEASURE q1",
    ];

    let gates = parse_lines(&input);
    let qasm = emit_qasm(&gates);
    // println!("{:#?}", gates);
    // println!("{}", qasm);

    fs::write("output.qasm", &qasm).expect("Unable to write file");
    println!("QASM code written to output.qasm");
}
