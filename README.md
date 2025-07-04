# Rust to QASM Transpiler

A minimal transpiler that converts a simple quantum assembly-like language to OpenQASM 2.0, implemented in Rust.

## Features

- Converts quantum assembly instructions to OpenQASM 2.0
- Supports basic quantum gates: H (Hadamard), X (Pauli-X), Z (Pauli-Z), CX (CNOT)
- Supports qubit allocation and measurement
- Generates valid OpenQASM 2.0 code

## Supported Instructions

| Instruction | Description | Example |
|-------------|-------------|----------|
| `qubit q0` | Allocates a quantum register | `qubit q0` |
| `H q0` | Applies Hadamard gate | `H q0` |
| `X q0` | Applies Pauli-X gate | `X q0` |
| `Z q0` | Applies Pauli-Z gate | `Z q0` |
| `CX q0 q1` | Applies CNOT gate (control, target) | `CX q0 q1` |
| `MEASURE q0` | Measures qubit and stores result in classical register | `MEASURE q0` |

## Example

Input (`bell_pair.txt`):
```
# Bell State: |Φ⁺⟩ = (|00⟩ + |11⟩)/√2
qubit q0
qubit q1
H q0
CX q0 q1
MEASURE q0
MEASURE q1
```

Output (`output.qasm`):
```qasm
OPENQASM 2.0;
include "qelib1.inc";

qreg q[2];
creg c[2];

h q[0];
cx q[0],q[1];
measure q[0] -> c[0];
measure q[1] -> c[1];
```

## Getting Started

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/rust-to-qasm-transpiler.git
   cd rust-to-qasm-transpiler
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run the example:
   ```bash
   cargo run
   ```

4. Check the generated `output.qasm` file.

## Blog Post

Read more about the implementation details and my experience building this transpiler in my Medium article:

[Building a Simple Rust to QASM Transpiler](https://medium.com/@hopesp444/building-a-simple-rust-to-qasm-transpiler-18e6f95db1c4)

## Project Structure

```
.
├── Cargo.toml          # Project manifest
├── src/
│   ├── main.rs        # Entry point
│   ├── lib.rs          # Library root
│   ├── parser.rs       # Parses input lines into Gate enum
│   ├── emitter.rs      # Converts Gate enum to QASM
│   └── gate.rs         # Gate enum definition
└── examples/
    └── bell.txt      # Example quantum circuits
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License 

## Acknowledgements

- OpenQASM for the quantum assembly standard
- The Rust community for excellent tooling and libraries