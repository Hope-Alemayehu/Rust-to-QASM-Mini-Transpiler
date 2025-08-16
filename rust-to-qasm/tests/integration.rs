use rust_to_qasm::{emit_qasm, parse_lines};

#[test]
fn test_basic_gates() {
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
    assert!(qasm.contains("z q[0];"));
    assert!(qasm.contains("measure q[0] -> c[0];"));
    assert!(qasm.contains("measure q[1] -> c[1];"));
}

#[test]
fn test_ccx_gate() {
    let input = vec![
        "qubit q0",
        "qubit q1",
        "qubit q2",
        "X q0",
        "X q1",
        "CCX q0 q1 q2",
        "MEASURE q2",
    ];

    let gates = parse_lines(&input);
    let qasm = emit_qasm(&gates);

    assert!(qasm.contains("x q[0];"));
    assert!(qasm.contains("x q[1];"));
    assert!(qasm.contains("ccx q[0],q[1],q[2];"));
    assert!(qasm.contains("measure q[2] -> c[0];"));
}

#[test]
fn test_rotation_gates() {
    let input = vec![
        "qubit q0",
        "qubit q1",
        "qubit q2",
        "RZ(1.57) q0",
        "RX(0.785) q1",
        "RY(3.14) q2",
        "MEASURE q0",
        "MEASURE q1",
        "MEASURE q2",
    ];

    let gates = parse_lines(&input);
    let qasm = emit_qasm(&gates);

    assert!(qasm.contains("rz(1.57) q[0];"));
    assert!(qasm.contains("rx(0.785) q[1];"));
    assert!(qasm.contains("ry(3.14) q[2];"));
    assert!(qasm.contains("measure q[0] -> c[0];"));
    assert!(qasm.contains("measure q[1] -> c[1];"));
    assert!(qasm.contains("measure q[2] -> c[2];"));
}

#[test]
fn test_expression_parameters() {
    let input = vec![
        "qubit q0",
        "RZ(pi/2) q0",
        "RX(2*theta) q0",
        "RY(1.5*pi) q0",
    ];

    let gates = parse_lines(&input);
    let qasm = emit_qasm(&gates);

    assert!(qasm.contains("rz(pi/2) q[0];"));
    assert!(qasm.contains("rx(2*theta) q[0];"));
    assert!(qasm.contains("ry(1.5*pi) q[0];"));
}
