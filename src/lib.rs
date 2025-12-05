//! Formal Circuit Minimization Compiler (FCMC)
//! A compiler for transforming high-level code to optimized arithmetic circuits for ZKPs

pub mod frontend;
pub mod ir;
pub mod optimization;
pub mod backend;
pub mod language;
pub mod utils;

pub use frontend::{compile_source, parse_source};
pub use ir::{Circuit, IRGraph};
pub use optimization::OptimizationFramework;
pub use backend::{TargetSystem, compile_to_target};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FCMCError {
    #[error("Parsing error: {0}")]
    ParseError(String),
    
    #[error("Type error: {0}")]
    TypeError(String),
    
    #[error("Semantic error: {0}")]
    SemanticError(String),
    
    #[error("Optimization error: {0}")]
    OptimizationError(String),
    
    #[error("Backend error: {0}")]
    BackendError(String),
    
    #[error("Verification error: {0}")]
    VerificationError(String),
}

/// Main compiler interface
pub struct FCMC {
    optimization_level: u8,
    target_system: TargetSystem,
    verify_output: bool,
}

impl FCMC {
    pub fn new() -> Self {
        Self {
            optimization_level: 2,
            target_system: TargetSystem::R1CS,
            verify_output: true,
        }
    }
    
    pub fn with_optimization_level(mut self, level: u8) -> Self {
        self.optimization_level = level;
        self
    }
    
    pub fn with_target(mut self, target: TargetSystem) -> Self {
        self.target_system = target;
        self
    }
    
    pub fn compile(&self, source: &str) -> Result<CompiledCircuit, FCMCError> {
        log::info!("Starting compilation with optimization level {}", self.optimization_level);
        
        // 1. Frontend: Parse and semantic analysis
        let ast = frontend::parse_source(source)?;
        log::debug!("AST generated successfully");
        
        // 2. Generate initial IR
        let mut ir = ir::IRGraph::from_ast(&ast)?;
        log::debug!("Initial IR generated with {} nodes", ir.node_count());
        
        // 3. Apply optimizations
        if self.optimization_level > 0 {
            let mut optimizer = optimization::OptimizationFramework::new();
            optimizer.set_level(self.optimization_level);
            ir = optimizer.optimize(ir)?;
            log::debug!("Optimized IR with {} nodes", ir.node_count());
        }
        
        // 4. Backend compilation
        let circuit = backend::compile_to_target(&ir, self.target_system)?;
        log::info!("Circuit compiled successfully with {} constraints", circuit.constraint_count());
        
        // 5. Verification if enabled
        if self.verify_output {
            utils::verification::verify_circuit(&circuit)?;
            log::debug!("Circuit verification passed");
        }
        
        Ok(CompiledCircuit {
            ir,
            circuit,
            stats: CompilationStats {
                original_nodes: 0, // Would be tracked
                optimized_nodes: ir.node_count(),
                constraint_count: circuit.constraint_count(),
            },
        })
    }
}

pub struct CompiledCircuit {
    pub ir: ir::IRGraph,
    pub circuit: Box<dyn backend::CircuitBackend>,
    pub stats: CompilationStats,
}

pub struct CompilationStats {
    pub original_nodes: usize,
    pub optimized_nodes: usize,
    pub constraint_count: usize,
}

impl CompiledCircuit {
    pub fn optimization_ratio(&self) -> f64 {
        if self.stats.original_nodes > 0 {
            let reduction = self.stats.original_nodes as f64 - self.stats.optimized_nodes as f64;
            reduction / self.stats.original_nodes as f64 * 100.0
        } else {
            0.0
        }
    }
}
