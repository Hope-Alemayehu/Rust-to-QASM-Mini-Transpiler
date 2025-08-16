#[derive(Debug, Clone)]
pub enum Gate {
    Qubit(String),
    H(String),
    X(String),
    Z(String),
    CX(String, String),
    CCX(String, String, String),  // Toffoli gate (CCNOT)
    RZ(String, String),           // Rotation around Z-axis (angle, qubit)
    RX(String, String),           // Rotation around X-axis (angle, qubit)
    RY(String, String),           // Rotation around Y-axis (angle, qubit)
    Measure(String),
}
