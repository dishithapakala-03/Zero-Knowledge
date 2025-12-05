# Formal Circuit Minimization Compiler (FCMC)

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Paper](https://img.shields.io/badge/Paper-IEEE%20Format-brightgreen)](paper.pdf)

A novel compiler framework that bridges the gap between high-level programming languages and zero-knowledge proof systems by automatically transforming conventional code into optimized arithmetic circuits for NP-complete statements.

> **Research Paper:** *"A Novel Compiler Design for Zero-Knowledge Proof Circuits: Bridging High-Level Code to NP-Complete Statements"*  
> Dishitha Pakala, M.S. - Kennesaw State University (2025)

## 🚀 Features

### 🔧 Core Compilation Pipeline
- **High-Level Language (L_ZK)**: Statically-typed DSL with formal semantics
- **Proof-System-Agnostic IR**: Intermediate representation enabling cross-system optimization
- **Algebraic Optimization Framework**: Systematic reduction of multiplicative complexity
- **Multiple Backend Targets**: R1CS, Plonk, AIR, and Halo2 support

### 📊 Optimization Capabilities
- **Common Subexpression Elimination**: Reduce redundant computations
- **Algebraic Simplification**: Apply distributive law, factorization, and identity rules
- **Constant Folding & Propagation**: Evaluate expressions at compile time
- **Circuit Size Reduction**: Up to 40% reduction in multiplication gates
- **Formal Verification**: SMT-based circuit correctness validation

### 🎯 Target Applications
- Privacy-preserving smart contracts
- Verifiable computation offloading
- Anonymous credentials and voting systems
- Cryptographic protocol verification
- Zero-knowledge machine learning

## 📋 Table of Contents
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Language Specification](#language-specification)
- [Architecture](#architecture)
- [Examples](#examples)
- [Benchmarks](#benchmarks)
- [API Documentation](#api-documentation)
- [Research](#research)
- [Contributing](#contributing)
- [License](#license)

## 🛠 Installation

### Prerequisites
- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Cargo package manager
- LLVM 15+ (for advanced optimizations)
- Z3 Theorem Prover (for formal verification)

### From Source
```bash
# Clone the repository
git clone https://github.com/yourusername/fcmc-compiler.git
cd fcmc-compiler

# Build in release mode
cargo build --release

# Install globally
cargo install --path .
```

### Via Cargo
```bash
cargo add fcmc-compiler
```

## 🚀 Quick Start

### Basic Usage
```bash
# Compile a ZK circuit
fcmc compile examples/sha256.fcmc --target r1cs --optimize 3

# Verify a compiled circuit
fcmc verify circuit.bin

# View language specification
fcmc spec
```

### Program Example (`simple.fcmc`)
```rust
// Simple zero-knowledge computation
fn main(public x: field, private y: field) -> field {
    // Common subexpression will be optimized
    let a = x * y + x * y;
    let b = (x + y) * (x - y);
    
    // Range check constraint
    constraint in_range(x: field) {
        x >= 0 && x < 100
    }
    
    assert in_range(x);
    return a + b;
}
```

### Rust API
```rust
use fcmc_compiler::{FCMC, TargetSystem};

let compiler = FCMC::new()
    .with_optimization_level(3)
    .with_target(TargetSystem::R1CS);

let source = r#"
    fn hash(public input: field, private salt: field) -> field {
        // Poseidon hash implementation
        return poseidon_hash(input, salt);
    }
"#;

let circuit = compiler.compile(source)?;
println!("Circuit compiled with {} constraints", circuit.constraint_count());
println!("Optimization ratio: {:.2}%", circuit.optimization_ratio());
```

## 📚 Language Specification

### Type System
```rust
// Primitive types
let x: field = 123;      // Finite field element
let b: bool = true;      // Boolean (encoded as field element)
let n: u32 = 42;         // 32-bit unsigned integer
let arr: [field; 4];     // Fixed-size array

// Struct definitions
struct Point {
    x: field,
    y: field,
}
```

### Control Flow
```rust
// Bounded loops (compile-time unrolling)
for i in 0..10 {
    result = result + i;
}

// Conditional statements
if x > y {
    return x;
} else {
    return y;
}
```

### Constraints
```rust
// Native constraint definitions
constraint range_check(value: field, bits: u32) {
    value >= 0 && value < (1 << bits)
}

constraint merkle_proof(root: field, leaf: field, path: [field]) {
    // Merkle tree verification logic
}
```

## 🏗 Architecture

### System Overview
```
┌─────────────────────────────────────────────────────┐
│                High-Level Program (L_ZK)            │
└──────────────────────────┬──────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────┐
│                  Frontend Compiler                  │
│  • Lexical Analysis & Parsing                      │
│  • Semantic Analysis & Type Checking               │
│  • AST Generation                                  │
└──────────────────────────┬──────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────┐
│         Intermediate Representation (IR)            │
│  • Proof-System-Agnostic Graph Representation      │
│  • SSA Form for Optimization                        │
│  • Dependency Tracking                              │
└──────────────────────────┬──────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────┐
│            Algebraic Optimization (𝒮)              │
│  • Common Subexpression Elimination                │
│  • Algebraic Simplification Rules                  │
│  • Constant Propagation & Folding                  │
│  • Circuit Size Minimization                       │
└──────────────────────────┬──────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────┐
│                 Backend Compilation                 │
│  • R1CS Constraint System Generation               │
│  • Plonkish Custom Gates                           │
│  • AIR Execution Traces                            │
│  • Halo2 Lookup Tables                             │
└──────────────────────────┬──────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────┐
│                 Optimized Circuit                   │
│  • Formal Verification                              │
│  • Proof Generation Ready                           │
└─────────────────────────────────────────────────────┘
```

### Optimization Framework
The compiler implements 10+ algebraic transformation rules:

| Rule | Pattern | Transformation | Complexity Reduction |
|------|---------|----------------|----------------------|
| Distributive Law | `a*b + a*c` | `a*(b + c)` | 2→1 multiplication |
| Multiplicative Identity | `x * 1` | `x` | Eliminates gate |
| Common Subexpression | `f(a,b)` × n | Single computation | n→1 computation |
| Constant Folding | `3 + 4` | `7` | Compile-time eval |
| Strength Reduction | `x * 16` | `x << 4` | Mul→Shift |

## 📊 Benchmarks

### Performance Comparison
```
Benchmark            | Gates (Orig) | Gates (Opt) | Reduction | Proof Time
---------------------|--------------|-------------|-----------|-----------
SHA-256              | 27,904       | 18,256      | 34.6%     | 1.2s → 0.8s
Merkle Proof (32)    | 4,192        | 2,875       | 31.4%     | 0.4s → 0.3s
Poseidon Hash        | 288          | 192         | 33.3%     | 0.1s → 0.07s
Elliptic Curve Add   | 784          | 512         | 34.7%     | 0.3s → 0.2s
```

### Compilation Speed
```
File Size | Parse Time | Opt Time | Total Time | Memory
----------|------------|----------|------------|--------
1 KB      | 12ms       | 45ms     | 67ms       | 8 MB
10 KB     | 98ms       | 210ms    | 328ms      | 24 MB
100 KB    | 850ms      | 1.2s     | 2.1s       | 128 MB
```

## 📖 Examples

### 1. Voting System
```rust
// examples/voting.fcmc
struct Vote {
    candidate_id: field,
    nullifier: field,
    secret: field,
}

fn verify_vote(public root: field, private vote: Vote, proof: [field; 8]) -> bool {
    // Merkle inclusion proof
    constraint merkle_inclusion(root: field, leaf: field, path: [field; 32]) {
        // Verification logic
    }
    
    // Nullifier hash check
    let computed_nullifier = poseidon_hash(vote.secret);
    assert computed_nullifier == vote.nullifier;
    
    // Range check for candidate ID
    constraint valid_candidate(id: field) {
        id >= 0 && id < 10
    }
    
    assert valid_candidate(vote.candidate_id);
    return merkle_inclusion(root, poseidon_hash(vote), proof);
}
```

### 2. Privacy-Preserving Payment
```rust
// examples/payment.fcmc
fn verify_payment(
    public balance_before: field,
    public balance_after: field,
    private amount: field,
    private blinding: field
) -> bool {
    // Pedersen commitment
    let commitment = pedersen_commit(amount, blinding);
    
    // Balance consistency
    assert balance_after == balance_before - amount;
    
    // Amount range check
    constraint positive_amount(a: field) {
        a > 0 && a < 1000000
    }
    
    assert positive_amount(amount);
    return true;
}
```

## 🔬 Research & Publications

### Academic Paper
This implementation accompanies the research paper:
- **Title**: "A Novel Compiler Design for Zero-Knowledge Proof Circuits: Bridging High-Level Code to NP-Complete Statements"
- **Author**: Dishitha Pakala, M.S.
- **Institution**: Kennesaw State University
- **Year**: 2025
- **DOI**: [To be assigned]

### Key Contributions
1. **Formal Language Specification**: Rigorous definition of L_ZK with guaranteed polynomial-sized circuit generation
2. **Optimization Framework**: Algebraic transformation system with formal correctness proofs
3. **Performance Evaluation**: Empirical validation showing 30-40% circuit size reduction
4. **Modular Architecture**: Extensible backend support for multiple proof systems

### Citing This Work
```bibtex
@inproceedings{pakala2025fcmc,
  title={A Novel Compiler Design for Zero-Knowledge Proof Circuits: 
         Bridging High-Level Code to NP-Complete Statements},
  author={Pakala, Dishitha},
  booktitle={IEEE Conference on Computer Science},
  year={2025},
  organization={IEEE}
}
```

## 🧪 Testing

### Unit Tests
```bash
cargo test --lib
```

### Integration Tests
```bash
cargo test --test integration
```

### Property-Based Testing
```bash
cargo test --features proptest
```

### Benchmark Suite
```bash
cargo bench
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup
```bash
# Clone and setup
git clone https://github.com/yourusername/fcmc-compiler.git
cd fcmc-compiler

# Install development dependencies
cargo install cargo-watch
cargo install cargo-tarpaulin  # For code coverage
cargo install cargo-audit       # For security audit

# Run tests
cargo test --all-features

# Run benchmarks
cargo bench
```

### Project Structure
```
fcmc-compiler/
├── src/                    # Source code
│   ├── frontend/          # Lexer, parser, semantic analysis
│   ├── ir/               # Intermediate representation
│   ├── optimization/      # Algebraic optimization passes
│   ├── backend/          # Target system compilation
│   ├── language/         # Language specification
│   └── utils/            # Utilities and helpers
├── examples/             # Example programs
├── tests/               # Test suite
├── benchmarks/          # Performance benchmarks
├── docs/               # Documentation
└── research/           # Research materials
```

## 📄 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## 📞 Contact

**Dishitha Pakala**  
- Email: [dishithapakala@gmail.com](mailto:dishithapakala@gmail.com)
- GitHub: [@yourusername](https://github.com/yourusername)
- LinkedIn: [Dishitha Pakala](https://linkedin.com/in/dishitha-pakala)

**Institution**  
Kennesaw State University  
Department of Computer Science  
Georgia, USA

## 🙏 Acknowledgments

- Bellman library developers for R1CS support
- ZCash team for foundational zk-SNARK research
- Stanford University's ZKProof community standards
- All contributors and testers of the FCMC compiler

---

<div align="center">
  <em>Making zero-knowledge proofs accessible and efficient for everyone</em>
</div>
