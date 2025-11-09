// Probabilistic edge management with PGM (Probabilistic Graph Memory) fields
//
// Edges represent relationships between entities that evolve based on access patterns.
// The probability field is updated using Kolmogorov probability theory.

use crate::core::types::EntityId;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Edge represents a probabilistic relationship between two entities
/// 
/// Mathematical properties (PGM):
/// - Probability: Weight in range [0.0, 1.0], updated on co-access
/// - Access count: Atomic counter for lock-free updates
/// - Last accessed: Timestamp for decay calculations
/// 
/// Probability update formula:
/// w_new = w_old + α * (co_access_frequency - w_old)
/// where α is learning rate (default 0.1)
/// 
/// Normalization constraint:
/// Σ P(edges from node_i) = 1.0 (within 0.001 tolerance)
#[derive(Debug, Serialize, Deserialize)]
pub struct Edge {
    /// Source entity ID
    pub source_id: EntityId,
    
    /// Target entity ID
    pub target_id: EntityId,
    
    /// Edge label (relationship type)
    pub label: String,
    
    /// Static weight (user-defined or initial weight)
    pub weight: f32,
    
    /// Optional metadata for the edge
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    
    // PGM (Probabilistic Graph Memory) fields
    
    /// Probabilistic weight learned from access patterns
    /// Range: [0.0, 1.0]
    /// Updated on co-access within 100ms time window
    pub probability: f32,
    
    /// Access count for learning (atomic for lock-free updates)
    /// Note: Serialization converts to u64, deserialization creates new AtomicU64
    #[serde(
        serialize_with = "serialize_atomic_u64",
        deserialize_with = "deserialize_atomic_u64"
    )]
    pub access_count: AtomicU64,
    
    /// Last accessed timestamp (Unix epoch milliseconds)
    /// Used for decay calculations and pruning
    #[serde(
        serialize_with = "serialize_atomic_u64",
        deserialize_with = "deserialize_atomic_u64"
    )]
    pub last_accessed: AtomicU64,
}

// Manual Clone implementation because AtomicU64 doesn't implement Clone
impl Clone for Edge {
    fn clone(&self) -> Self {
        Self {
            source_id: self.source_id,
            target_id: self.target_id,
            label: self.label.clone(),
            weight: self.weight,
            metadata: self.metadata.clone(),
            probability: self.probability,
            access_count: AtomicU64::new(self.access_count.load(Ordering::SeqCst)),
            last_accessed: AtomicU64::new(self.last_accessed.load(Ordering::SeqCst)),
        }
    }
}

impl Edge {
    /// Create a new Edge with initial probability
    /// 
    /// # Arguments
    /// * `source_id` - Source entity ID
    /// * `target_id` - Target entity ID
    /// * `label` - Edge label (relationship type)
    /// * `weight` - Initial static weight
    /// * `metadata` - Optional metadata
    /// 
    /// # Returns
    /// * Edge with probability initialized to weight, access_count = 0
    pub fn new(
        source_id: EntityId,
        target_id: EntityId,
        label: String,
        weight: f32,
        metadata: Option<serde_json::Value>,
    ) -> Self {
        // Ensure weight is in valid range
        let weight = weight.clamp(0.0, 1.0);
        
        Self {
            source_id,
            target_id,
            label,
            weight,
            metadata,
            probability: weight, // Initialize probability to weight
            access_count: AtomicU64::new(0),
            last_accessed: AtomicU64::new(current_timestamp_ms()),
        }
    }

    /// Record an access to this edge (lock-free)
    /// 
    /// Increments access_count and updates last_accessed timestamp atomically.
    /// This is called when entities are co-accessed within the time window.
    pub fn record_access(&self) {
        self.access_count.fetch_add(1, Ordering::SeqCst);
        self.last_accessed.store(current_timestamp_ms(), Ordering::SeqCst);
    }

    /// Get the current access count
    pub fn get_access_count(&self) -> u64 {
        self.access_count.load(Ordering::SeqCst)
    }

    /// Get the last accessed timestamp (milliseconds since Unix epoch)
    pub fn get_last_accessed(&self) -> u64 {
        self.last_accessed.load(Ordering::SeqCst)
    }

    /// Update probability based on learning algorithm
    /// 
    /// Formula: w_new = w_old + α * (target - w_old)
    /// 
    /// # Arguments
    /// * `target` - Target probability (e.g., normalized co-access frequency)
    /// * `learning_rate` - Learning rate α (default 0.1)
    /// 
    /// # Returns
    /// * New probability value
    pub fn update_probability(&mut self, target: f32, learning_rate: f32) -> f32 {
        let target = target.clamp(0.0, 1.0);
        let learning_rate = learning_rate.clamp(0.0, 1.0);
        
        // Apply learning update
        self.probability = self.probability + learning_rate * (target - self.probability);
        self.probability = self.probability.clamp(0.0, 1.0);
        
        self.probability
    }

    /// Check if edge should be pruned based on threshold and inactivity
    /// 
    /// Pruning criteria:
    /// - Probability below threshold (default 0.01)
    /// - No access for 30 days
    /// 
    /// # Arguments
    /// * `threshold` - Minimum probability to keep edge
    /// * `inactivity_days` - Days of inactivity before pruning
    /// 
    /// # Returns
    /// * true if edge should be pruned
    pub fn should_prune(&self, threshold: f32, inactivity_days: u64) -> bool {
        // Check probability threshold
        if self.probability < threshold {
            return true;
        }
        
        // Check inactivity
        let last_accessed = self.get_last_accessed();
        let current = current_timestamp_ms();
        let inactivity_ms = inactivity_days * 24 * 60 * 60 * 1000;
        
        if current - last_accessed > inactivity_ms {
            return true;
        }
        
        false
    }

    /// Decay probability over time (for edges not accessed)
    /// 
    /// Applies exponential decay: p_new = p_old * exp(-λ * Δt)
    /// 
    /// # Arguments
    /// * `decay_rate` - Decay rate λ (default 0.01 per day)
    /// 
    /// # Returns
    /// * New probability after decay
    pub fn apply_decay(&mut self, decay_rate: f32) -> f32 {
        let last_accessed = self.get_last_accessed();
        let current = current_timestamp_ms();
        let days_elapsed = (current - last_accessed) as f32 / (24.0 * 60.0 * 60.0 * 1000.0);
        
        // Exponential decay
        let decay_factor = (-decay_rate * days_elapsed).exp();
        self.probability *= decay_factor;
        self.probability = self.probability.clamp(0.0, 1.0);
        
        self.probability
    }
}

/// Get current timestamp in milliseconds since Unix epoch
fn current_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
}

/// Serialize AtomicU64 as u64
fn serialize_atomic_u64<S>(atomic: &AtomicU64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_u64(atomic.load(Ordering::SeqCst))
}

/// Deserialize u64 as AtomicU64
fn deserialize_atomic_u64<'de, D>(deserializer: D) -> Result<AtomicU64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = u64::deserialize(deserializer)?;
    Ok(AtomicU64::new(value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_creation() {
        let source = EntityId::new();
        let target = EntityId::new();
        let edge = Edge::new(
            source,
            target,
            "related_to".to_string(),
            0.5,
            None,
        );
        
        assert_eq!(edge.source_id, source);
        assert_eq!(edge.target_id, target);
        assert_eq!(edge.label, "related_to");
        assert_eq!(edge.weight, 0.5);
        assert_eq!(edge.probability, 0.5);
        assert_eq!(edge.get_access_count(), 0);
    }

    #[test]
    fn test_record_access() {
        let edge = Edge::new(
            EntityId::new(),
            EntityId::new(),
            "test".to_string(),
            0.5,
            None,
        );
        
        assert_eq!(edge.get_access_count(), 0);
        
        edge.record_access();
        assert_eq!(edge.get_access_count(), 1);
        
        edge.record_access();
        assert_eq!(edge.get_access_count(), 2);
    }

    #[test]
    fn test_update_probability() {
        let mut edge = Edge::new(
            EntityId::new(),
            EntityId::new(),
            "test".to_string(),
            0.5,
            None,
        );
        
        // Update towards 0.8 with learning rate 0.1
        // new = 0.5 + 0.1 * (0.8 - 0.5) = 0.5 + 0.03 = 0.53
        let new_prob = edge.update_probability(0.8, 0.1);
        assert!((new_prob - 0.53).abs() < 1e-6);
        assert_eq!(edge.probability, new_prob);
    }

    #[test]
    fn test_probability_bounds() {
        let mut edge = Edge::new(
            EntityId::new(),
            EntityId::new(),
            "test".to_string(),
            0.5,
            None,
        );
        
        // Try to update beyond bounds
        edge.update_probability(1.5, 0.5); // Should clamp to 1.0
        assert!(edge.probability <= 1.0);
        
        edge.update_probability(-0.5, 0.5); // Should clamp to 0.0
        assert!(edge.probability >= 0.0);
    }

    #[test]
    fn test_should_prune_threshold() {
        let mut edge = Edge::new(
            EntityId::new(),
            EntityId::new(),
            "test".to_string(),
            0.005,
            None,
        );
        
        edge.probability = 0.005;
        
        // Should prune if below threshold
        assert!(edge.should_prune(0.01, 30));
        
        // Should not prune if above threshold
        edge.probability = 0.02;
        assert!(!edge.should_prune(0.01, 30));
    }

    #[test]
    fn test_apply_decay() {
        let mut edge = Edge::new(
            EntityId::new(),
            EntityId::new(),
            "test".to_string(),
            1.0,
            None,
        );
        
        edge.probability = 1.0;
        
        // Simulate 1 day passing by setting last_accessed to 1 day ago
        let one_day_ago = current_timestamp_ms() - (24 * 60 * 60 * 1000);
        edge.last_accessed.store(one_day_ago, Ordering::SeqCst);
        
        // Apply decay with rate 0.01 per day
        // new = 1.0 * exp(-0.01 * 1) ≈ 0.99
        let new_prob = edge.apply_decay(0.01);
        assert!((new_prob - 0.99).abs() < 0.01);
    }

    #[test]
    fn test_edge_serialization() {
        let edge = Edge::new(
            EntityId::new(),
            EntityId::new(),
            "test".to_string(),
            0.5,
            Some(serde_json::json!({"key": "value"})),
        );
        
        edge.record_access();
        edge.record_access();
        
        let serialized = serde_json::to_string(&edge).unwrap();
        let deserialized: Edge = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(edge.source_id, deserialized.source_id);
        assert_eq!(edge.target_id, deserialized.target_id);
        assert_eq!(edge.label, deserialized.label);
        assert_eq!(edge.probability, deserialized.probability);
        assert_eq!(edge.get_access_count(), deserialized.get_access_count());
    }
}
