#[derive(Debug, Clone)]
pub enum Gate {
    Qubit(String),
    H(String),
    X(String),
    Z(String),
    CX(String, String),
    Measure(String),
}
