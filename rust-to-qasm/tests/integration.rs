use rust_to_qasm::{emit_qasm, parse_lines};

#[test]

fn test_full_transpile() {
    let input = vec![
        "qubit q0",
        "qubit q1",
        " H q0",
        "CX q0 q1",
        "Z q0",
        "MEASURE q0",
        "MEASURE q1",
    ];

    let gates = parse_lines(&input);
    let qasm = emit_qasm(&gates);

    assert!(qasm.contains("h q[0];"));
    assert!(qasm.contains("cx q[0],q[1];"));
    assert!(qasm.contains("measure q[0] -> c[0];"));
}
