# Zero-Knowledge
A Novel Compiler Design for Zero-Knowledge Proof Circuits: Bridging High-Level Code to NP-Complete Statements
fcmc/
├── Cargo.toml                          # Project configuration & dependencies
├── Cargo.lock                          # Dependency lock file (generated)
├── README.md                           # Main project documentation
├── LICENSE                             # MIT License
├── PROJECT_STRUCTURE.md                # This file
│
├── src/                                # Source code
│   ├── lib.rs                          # Main library entry (created)
│   ├── main.rs                         # CLI application (created)
│   │
│   ├── frontend/                       # Frontend components
│   │   ├── mod.rs                      # Frontend module (created)
│   │   ├── lexer.rs                    # Lexical analysis (in mod.rs)
│   │   ├── parser.rs                   # Syntax parsing (in mod.rs)
│   │   └── semantic.rs                 # Type checking (in mod.rs)
│   │
│   ├── ir/                             # Intermediate Representation
│   │   ├── mod.rs                      # IR module (created)
│   │   └── generator.rs                # IR generation (in mod.rs)
│   │
│   ├── optimization/                   # Optimization framework
│   │   ├── mod.rs                      # Optimization module (created)
│   │   ├── algebraic.rs                # Algebraic transformations (in mod.rs)
│   │   ├── cse.rs                      # Common subexpression elimination (in mod.rs)
│   │   └── strength_reduction.rs      # Strength reduction optimizations
│   │
│   ├── backend/                        # Backend compilation
│   │   ├── mod.rs                      # Backend module (created)
│   │   ├── r1cs.rs                     # R1CS compilation (in mod.rs)
│   │   ├── plonk.rs                    # Plonk compilation (in mod.rs)
│   │   ├── air.rs                      # AIR compilation (in mod.rs)
│   │   └── halo2.rs                    # Halo2 compilation (in mod.rs)
│   │
│   ├── types.rs                        # Common type definitions
│   └── utils.rs                        # Utility functions
│
├── examples/                           # Example programs
│   ├── main.rs                         # Example runner (created)
│   ├── sha256.lzk                      # SHA-256 circuit example
│   ├── merkle.lzk                      # Merkle tree verification
│   ├── poseidon.lzk                    # Poseidon hash function
│   └── ecdsa.lzk                       # ECDSA signature verification
│
├── tests/                              # Test suite
│   ├── integration_tests.rs            # Integration tests
│   ├── property_tests.rs               # Property-based tests
│   ├── frontend_tests.rs               # Frontend testing
│   ├── optimization_tests.rs           # Optimization testing
│   └── backend_tests.rs                # Backend testing
│
├── benches/                            # Benchmarks
│   ├── optimization_bench.rs           # Optimization performance
│   ├── compilation_bench.rs            # Compilation speed
│   └── circuit_size_bench.rs           # Circuit size benchmarks
│
├── docs/                               # Documentation
│   ├── language_spec.md                # L_ZK language specification
│   ├── ir_format.md                    # IR format documentation
│   ├── optimization_passes.md          # Optimization pass details
│   ├── backend_targets.md              # Backend target specifications
│   └── api_reference.md                # API documentation
│
└── .github/                            # GitHub configuration
    ├── workflows/
    │   ├── ci.yml                      # Continuous integration
    │   └── release.yml                 # Release automation
    └── ISSUE_TEMPLATE/
        ├── bug_report.md               # Bug report template
        └── feature_request.md          # Feature request template
