// Core type aliases for Phenix-DB
//
// These type aliases provide semantic meaning and type safety throughout the system.
// They are based on UUID v7 for distributed generation with temporal ordering.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Unique identifier for an Entity in the memory substrate
/// 
/// EntityId uses UUID v7 format for:
/// - Distributed generation without coordination
/// - Temporal ordering (creation time embedded)
/// - 128-bit uniqueness guarantee
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId(uuid::Uuid);

impl EntityId {
    /// Generate a new EntityId with temporal ordering
    pub fn new() -> Self {
        Self(uuid::Uuid::now_v7())
    }

    /// Create EntityId from existing UUID
    pub fn from_uuid(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }

    /// Get the underlying UUID
    pub fn as_uuid(&self) -> &uuid::Uuid {
        &self.0
    }
}

impl Default for EntityId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for a Node in the distributed consciousness system
/// 
/// NodeId represents a physical or logical node in the cluster.
/// Each node maintains partial awareness of the global memory graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(uuid::Uuid);

impl NodeId {
    /// Generate a new NodeId
    pub fn new() -> Self {
        Self(uuid::Uuid::now_v7())
    }

    /// Create NodeId from existing UUID
    pub fn from_uuid(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }

    /// Get the underlying UUID
    pub fn as_uuid(&self) -> &uuid::Uuid {
        &self.0
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for a Shard in the memory substrate
/// 
/// ShardId represents a logical partition of the memory space.
/// Semantically similar entities are co-located in the same shard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ShardId(uuid::Uuid);

impl ShardId {
    /// Generate a new ShardId
    pub fn new() -> Self {
        Self(uuid::Uuid::now_v7())
    }

    /// Create ShardId from existing UUID
    pub fn from_uuid(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }

    /// Get the underlying UUID
    pub fn as_uuid(&self) -> &uuid::Uuid {
        &self.0
    }
}

impl Default for ShardId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ShardId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for a semantic Cluster in the memory substrate
/// 
/// ClusterId represents a group of semantically similar entities.
/// Entities with similarity >0.85 are grouped into the same cluster.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClusterId(uuid::Uuid);

impl ClusterId {
    /// Generate a new ClusterId
    pub fn new() -> Self {
        Self(uuid::Uuid::now_v7())
    }

    /// Create ClusterId from existing UUID
    pub fn from_uuid(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }

    /// Get the underlying UUID
    pub fn as_uuid(&self) -> &uuid::Uuid {
        &self.0
    }
}

impl Default for ClusterId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ClusterId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_id_creation() {
        let id1 = EntityId::new();
        let id2 = EntityId::new();
        assert_ne!(id1, id2, "EntityIds should be unique");
    }

    #[test]
    fn test_node_id_creation() {
        let id1 = NodeId::new();
        let id2 = NodeId::new();
        assert_ne!(id1, id2, "NodeIds should be unique");
    }

    #[test]
    fn test_shard_id_creation() {
        let id1 = ShardId::new();
        let id2 = ShardId::new();
        assert_ne!(id1, id2, "ShardIds should be unique");
    }

    #[test]
    fn test_cluster_id_creation() {
        let id1 = ClusterId::new();
        let id2 = ClusterId::new();
        assert_ne!(id1, id2, "ClusterIds should be unique");
    }

    #[test]
    fn test_id_serialization() {
        let id = EntityId::new();
        let serialized = serde_json::to_string(&id).unwrap();
        let deserialized: EntityId = serde_json::from_str(&serialized).unwrap();
        assert_eq!(id, deserialized, "EntityId should serialize/deserialize correctly");
    }
}
