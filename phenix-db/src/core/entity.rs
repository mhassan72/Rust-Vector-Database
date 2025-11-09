// Entity - Unified data structure for Phenix-DB
//
// Entity is the first-class cognitive unit containing:
// - Vector embeddings (high-dimensional representations)
// - Metadata (flexible JSONB)
// - Edges (probabilistic relationships)
// - Memory substrate fields (polynomial embeddings, access statistics)

use crate::core::edges::Edge;
use crate::core::types::EntityId;
use crate::core::vector::Vector;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Entity represents a unified data structure in the memory substrate
/// 
/// Design principles:
/// - Unified: Vector + metadata + edges in single ACID transaction
/// - Cognitive: Learns from access patterns and self-organizes
/// - Hierarchical: Stored as polynomial embeddings in RPI
/// - Adaptive: Moves between memory tiers based on access frequency
/// 
/// Requirements: 1.5, 2.5, 17.1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    /// Unique identifier
    pub id: EntityId,
    
    /// Optional vector embedding (128-4096 dimensions)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector: Option<Vector>,
    
    /// Optional metadata (flexible JSONB)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    
    /// Optional edges (probabilistic relationships)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<Edge>>,
    
    /// Creation timestamp (Unix epoch milliseconds)
    pub created_at: u64,
    
    /// Last update timestamp (Unix epoch milliseconds)
    pub updated_at: u64,
    
    /// MVCC version for concurrency control
    pub version: u64,
    
    /// Current memory tier (hot/warm/cold)
    pub tier: MemoryTier,
    
    // Mathematical memory substrate fields
    
    /// Polynomial embedding for RPI (Recursive Polynomial Index)
    /// Computed from vector, metadata, and edges
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polynomial_embedding: Option<PolynomialEmbedding>,
    
    /// Access statistics for adaptive learning
    pub access_statistics: AccessStatistics,
    
    /// Compression metadata for KCE (Kolmogorov Compression Engine)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_metadata: Option<CompressionMetadata>,
}

impl Entity {
    /// Create a new Entity with minimal fields
    /// 
    /// # Arguments
    /// * `vector` - Optional vector embedding
    /// * `metadata` - Optional metadata
    /// * `edges` - Optional edges
    /// 
    /// # Returns
    /// * Entity with generated ID, current timestamp, and default values
    pub fn new(
        vector: Option<Vector>,
        metadata: Option<serde_json::Value>,
        edges: Option<Vec<Edge>>,
    ) -> Self {
        let now = current_timestamp_ms();
        
        Self {
            id: EntityId::new(),
            vector,
            metadata,
            edges,
            created_at: now,
            updated_at: now,
            version: 1,
            tier: MemoryTier::Hot, // New entities start in hot tier
            polynomial_embedding: None,
            access_statistics: AccessStatistics::new(),
            compression_metadata: None,
        }
    }

    /// Record an access to this entity
    /// 
    /// Updates access statistics for adaptive learning and tiering decisions.
    pub fn record_access(&mut self) {
        self.access_statistics.record_access();
        self.updated_at = current_timestamp_ms();
    }

    /// Check if entity should be promoted to a higher tier
    /// 
    /// Promotion thresholds:
    /// - Cold → Warm: 10 accesses/hour
    /// - Warm → Hot: 100 accesses/hour
    pub fn should_promote(&self) -> bool {
        match self.tier {
            MemoryTier::Cold => {
                self.access_statistics.access_frequency >= 10.0
            }
            MemoryTier::Warm => {
                self.access_statistics.access_frequency >= 100.0
            }
            MemoryTier::Hot => false, // Already at highest tier
        }
    }

    /// Check if entity should be demoted to a lower tier
    /// 
    /// Demotion thresholds:
    /// - Hot → Warm: <1 access/hour for 24 hours
    /// - Warm → Cold: <0.1 access/hour for 7 days
    pub fn should_demote(&self) -> bool {
        let hours_since_access = self.access_statistics.hours_since_last_access();
        
        match self.tier {
            MemoryTier::Hot => {
                self.access_statistics.access_frequency < 1.0 && hours_since_access >= 24.0
            }
            MemoryTier::Warm => {
                self.access_statistics.access_frequency < 0.1 && hours_since_access >= 168.0 // 7 days
            }
            MemoryTier::Cold => false, // Already at lowest tier
        }
    }

    /// Promote entity to next higher tier
    pub fn promote(&mut self) {
        self.tier = match self.tier {
            MemoryTier::Cold => MemoryTier::Warm,
            MemoryTier::Warm => MemoryTier::Hot,
            MemoryTier::Hot => MemoryTier::Hot, // Already at highest
        };
        self.updated_at = current_timestamp_ms();
    }

    /// Demote entity to next lower tier
    pub fn demote(&mut self) {
        self.tier = match self.tier {
            MemoryTier::Hot => MemoryTier::Warm,
            MemoryTier::Warm => MemoryTier::Cold,
            MemoryTier::Cold => MemoryTier::Cold, // Already at lowest
        };
        self.updated_at = current_timestamp_ms();
    }
}

/// MemoryTier represents the hierarchical memory tier for an entity
/// 
/// Three-tier architecture:
/// - Hot: RAM/NVMe, <1ms access time, frequently accessed data
/// - Warm: NVMe/SSD, 1-10ms access time, moderately accessed data
/// - Cold: Object storage, 10-100ms access time, rarely accessed data (compressed)
/// 
/// Requirement: 15.1, 15.3
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryTier {
    /// Hot tier: RAM/NVMe storage
    /// - Latency: <1ms
    /// - Typical size: 1-10% of data
    /// - Access frequency: >100 accesses/hour
    Hot,
    
    /// Warm tier: NVMe/SSD storage
    /// - Latency: 1-10ms
    /// - Typical size: 10-30% of data
    /// - Access frequency: 10-100 accesses/hour
    Warm,
    
    /// Cold tier: Object storage (compressed)
    /// - Latency: 10-100ms
    /// - Typical size: 60-90% of data
    /// - Access frequency: <10 accesses/hour
    /// - Compression: 70-90% reduction via KCE
    Cold,
}

impl MemoryTier {
    /// Get the typical latency characteristics for this tier
    pub fn latency_characteristics(&self) -> Duration {
        match self {
            MemoryTier::Hot => Duration::from_micros(500),   // <1ms
            MemoryTier::Warm => Duration::from_millis(5),    // 1-10ms
            MemoryTier::Cold => Duration::from_millis(50),   // 10-100ms
        }
    }

    /// Get the promotion threshold (accesses per hour)
    pub fn promotion_threshold(&self) -> f32 {
        match self {
            MemoryTier::Cold => 10.0,
            MemoryTier::Warm => 100.0,
            MemoryTier::Hot => f32::INFINITY, // No promotion from hot
        }
    }

    /// Get the demotion threshold (accesses per hour)
    pub fn demotion_threshold(&self) -> f32 {
        match self {
            MemoryTier::Hot => 1.0,
            MemoryTier::Warm => 0.1,
            MemoryTier::Cold => 0.0, // No demotion from cold
        }
    }
}

/// AccessStatistics tracks entity access patterns for adaptive learning
/// 
/// Used for:
/// - Tier promotion/demotion decisions
/// - Bellman optimizer path optimization
/// - Adaptive learning predictions
/// - Semantic locality clustering
/// 
/// Requirement: 17.1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessStatistics {
    /// Total number of accesses
    pub total_accesses: u64,
    
    /// Last access timestamp (Unix epoch milliseconds)
    pub last_access: u64,
    
    /// Access frequency (accesses per hour)
    /// Computed using exponential moving average
    pub access_frequency: f32,
    
    /// Entities frequently co-accessed with this entity
    /// Used for PGM edge weight updates and semantic clustering
    pub co_access_entities: Vec<EntityId>,
}

impl AccessStatistics {
    /// Create new AccessStatistics with default values
    pub fn new() -> Self {
        Self {
            total_accesses: 0,
            last_access: current_timestamp_ms(),
            access_frequency: 0.0,
            co_access_entities: Vec::new(),
        }
    }

    /// Record an access and update statistics
    /// 
    /// Updates:
    /// - Increments total_accesses
    /// - Updates last_access timestamp
    /// - Recalculates access_frequency using exponential moving average
    pub fn record_access(&mut self) {
        let now = current_timestamp_ms();
        let hours_elapsed = (now - self.last_access) as f32 / (60.0 * 60.0 * 1000.0);
        
        self.total_accesses += 1;
        self.last_access = now;
        
        // Update access frequency using exponential moving average
        // α = 0.1 (smoothing factor)
        if hours_elapsed > 0.0 {
            let instant_frequency = 1.0 / hours_elapsed;
            self.access_frequency = 0.9 * self.access_frequency + 0.1 * instant_frequency;
        }
    }

    /// Record a co-access with another entity
    /// 
    /// Used for PGM edge weight updates and semantic clustering.
    /// Maintains a limited history (max 100 entities).
    pub fn record_co_access(&mut self, entity_id: EntityId) {
        // Add to co-access list if not already present
        if !self.co_access_entities.contains(&entity_id) {
            self.co_access_entities.push(entity_id);
            
            // Limit history to 100 entities
            if self.co_access_entities.len() > 100 {
                self.co_access_entities.remove(0);
            }
        }
    }

    /// Get hours since last access
    pub fn hours_since_last_access(&self) -> f32 {
        let now = current_timestamp_ms();
        (now - self.last_access) as f32 / (60.0 * 60.0 * 1000.0)
    }

    /// Check if entity is stagnant (no access for 90 days)
    pub fn is_stagnant(&self) -> bool {
        self.hours_since_last_access() >= 90.0 * 24.0 // 90 days
    }
}

impl Default for AccessStatistics {
    fn default() -> Self {
        Self::new()
    }
}

/// PolynomialEmbedding for RPI (Recursive Polynomial Index)
/// 
/// Stores entity as polynomial: P_k(V) = Σ a_i * V^i
/// Coefficients derived from vector, metadata, and edges.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolynomialEmbedding {
    /// Polynomial coefficients
    pub coefficients: Vec<f64>,
    
    /// Polynomial degree
    pub degree: usize,
    
    /// Hash of metadata for quick filtering
    pub metadata_hash: u64,
    
    /// Compact edge representation
    pub edge_signature: Vec<u8>,
}

/// CompressionMetadata for KCE (Kolmogorov Compression Engine)
/// 
/// Tracks compression information for cold tier storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionMetadata {
    /// Compression method used
    pub method: CompressionMethod,
    
    /// Original size in bytes
    pub original_size: usize,
    
    /// Compressed size in bytes
    pub compressed_size: usize,
    
    /// Compression ratio (compressed_size / original_size)
    pub compression_ratio: f32,
    
    /// Dictionary references (for pattern dictionary compression)
    pub dictionary_refs: Vec<u64>,
}

/// CompressionMethod used by KCE
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressionMethod {
    /// Ramanujan series encoding
    RamanujanSeries {
        coefficients: Vec<f64>,
        degree: usize,
    },
    
    /// Pattern dictionary compression
    PatternDictionary {
        refs: Vec<u64>,
    },
    
    /// Hybrid: Ramanujan + dictionary
    Hybrid {
        series: Vec<f64>,
        refs: Vec<u64>,
    },
    
    /// No compression
    None,
}

/// Get current timestamp in milliseconds since Unix epoch
fn current_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_creation() {
        let vector = Vector::new(vec![1.0, 2.0, 3.0]);
        let metadata = serde_json::json!({"key": "value"});
        
        let entity = Entity::new(
            Some(vector),
            Some(metadata),
            None,
        );
        
        assert!(entity.vector.is_some());
        assert!(entity.metadata.is_some());
        assert_eq!(entity.version, 1);
        assert_eq!(entity.tier, MemoryTier::Hot);
        assert_eq!(entity.access_statistics.total_accesses, 0);
    }

    #[test]
    fn test_record_access() {
        let mut entity = Entity::new(None, None, None);
        
        assert_eq!(entity.access_statistics.total_accesses, 0);
        
        entity.record_access();
        assert_eq!(entity.access_statistics.total_accesses, 1);
        
        entity.record_access();
        assert_eq!(entity.access_statistics.total_accesses, 2);
    }

    #[test]
    fn test_tier_promotion() {
        let mut entity = Entity::new(None, None, None);
        entity.tier = MemoryTier::Cold;
        entity.access_statistics.access_frequency = 15.0; // Above cold→warm threshold
        
        assert!(entity.should_promote());
        
        entity.promote();
        assert_eq!(entity.tier, MemoryTier::Warm);
        
        entity.access_statistics.access_frequency = 150.0; // Above warm→hot threshold
        assert!(entity.should_promote());
        
        entity.promote();
        assert_eq!(entity.tier, MemoryTier::Hot);
        
        // Can't promote from hot
        assert!(!entity.should_promote());
    }

    #[test]
    fn test_tier_demotion() {
        let mut entity = Entity::new(None, None, None);
        entity.tier = MemoryTier::Hot;
        entity.access_statistics.access_frequency = 0.5; // Below hot→warm threshold
        
        // Simulate 24 hours passing
        entity.access_statistics.last_access = current_timestamp_ms() - (24 * 60 * 60 * 1000);
        
        assert!(entity.should_demote());
        
        entity.demote();
        assert_eq!(entity.tier, MemoryTier::Warm);
    }

    #[test]
    fn test_memory_tier_latency() {
        assert!(MemoryTier::Hot.latency_characteristics() < Duration::from_millis(1));
        assert!(MemoryTier::Warm.latency_characteristics() < Duration::from_millis(10));
        assert!(MemoryTier::Cold.latency_characteristics() < Duration::from_millis(100));
    }

    #[test]
    fn test_access_statistics() {
        let mut stats = AccessStatistics::new();
        
        assert_eq!(stats.total_accesses, 0);
        assert_eq!(stats.access_frequency, 0.0);
        
        stats.record_access();
        assert_eq!(stats.total_accesses, 1);
        
        // Test co-access recording
        let entity_id = EntityId::new();
        stats.record_co_access(entity_id);
        assert_eq!(stats.co_access_entities.len(), 1);
        assert_eq!(stats.co_access_entities[0], entity_id);
    }

    #[test]
    fn test_stagnant_detection() {
        let mut stats = AccessStatistics::new();
        
        // Simulate 90 days passing
        stats.last_access = current_timestamp_ms() - (90 * 24 * 60 * 60 * 1000);
        
        assert!(stats.is_stagnant());
    }

    #[test]
    fn test_entity_serialization() {
        let vector = Vector::new(vec![1.0, 2.0, 3.0]);
        let entity = Entity::new(Some(vector), None, None);
        
        let serialized = serde_json::to_string(&entity).unwrap();
        let deserialized: Entity = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(entity.id, deserialized.id);
        assert_eq!(entity.tier, deserialized.tier);
        assert_eq!(entity.version, deserialized.version);
    }
}
