// Error types for Phenix-DB core module
//
// This module will be fully implemented in Task 3.
// Placeholder for now to allow compilation.

use thiserror::Error;

/// Result type alias for core operations
pub type Result<T> = std::result::Result<T, MemorySubstrateError>;

/// Error types for the memory substrate
#[derive(Debug, Error)]
pub enum MemorySubstrateError {
    #[error("Polynomial evaluation failed: {0}")]
    PolynomialError(String),
    
    #[error("Probabilistic graph operation failed: {0}")]
    GraphError(String),
    
    #[error("Compression failed: {0}")]
    CompressionError(String),
    
    #[error("Consensus not achieved: {0}")]
    ConsensusError(String),
    
    #[error("Tier operation failed: {0}")]
    TierError(String),
    
    #[error("Learning algorithm failed: {0}")]
    LearningError(String),
    
    #[error("Concurrency conflict: {0}")]
    ConcurrencyError(String),
    
    #[error("Mathematical invariant violated: {0}")]
    InvariantViolation(String),
}
