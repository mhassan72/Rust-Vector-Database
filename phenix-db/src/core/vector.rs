// Vector operations and distance functions
//
// Vector is a core component of the Entity, representing high-dimensional embeddings.
// The norm is precomputed for efficiency in distance calculations.

use serde::{Deserialize, Serialize};

/// Vector represents a high-dimensional embedding with precomputed norm
/// 
/// Mathematical properties:
/// - Dimensions: 128-4096 (configurable)
/// - Values: f32 for memory efficiency
/// - Norm: Precomputed L2 norm for fast distance calculations
/// 
/// The precomputed norm enables O(1) cosine similarity calculations:
/// cosine(v1, v2) = dot(v1, v2) / (norm(v1) * norm(v2))
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector {
    /// Number of dimensions in the vector
    pub dimensions: usize,
    
    /// Vector values (length must equal dimensions)
    pub values: Vec<f32>,
    
    /// Precomputed L2 norm: sqrt(sum(x_i^2))
    /// Cached for efficiency in distance calculations
    pub norm: f32,
}

impl Vector {
    /// Create a new Vector with automatic norm computation
    /// 
    /// # Arguments
    /// * `values` - Vector values (must not be empty)
    /// 
    /// # Returns
    /// * `Vector` with precomputed norm
    /// 
    /// # Panics
    /// * If values is empty
    pub fn new(values: Vec<f32>) -> Self {
        assert!(!values.is_empty(), "Vector values cannot be empty");
        
        let dimensions = values.len();
        let norm = Self::compute_norm(&values);
        
        Self {
            dimensions,
            values,
            norm,
        }
    }

    /// Create a zero vector with specified dimensions
    pub fn zeros(dimensions: usize) -> Self {
        Self {
            dimensions,
            values: vec![0.0; dimensions],
            norm: 0.0,
        }
    }

    /// Compute L2 norm: sqrt(sum(x_i^2))
    fn compute_norm(values: &[f32]) -> f32 {
        values.iter()
            .map(|&x| x * x)
            .sum::<f32>()
            .sqrt()
    }

    /// Recompute and update the cached norm
    /// 
    /// Call this after modifying vector values directly
    pub fn update_norm(&mut self) {
        self.norm = Self::compute_norm(&self.values);
    }

    /// Compute dot product with another vector
    /// 
    /// # Arguments
    /// * `other` - Vector to compute dot product with
    /// 
    /// # Returns
    /// * Dot product value
    /// 
    /// # Panics
    /// * If dimensions don't match
    pub fn dot(&self, other: &Vector) -> f32 {
        assert_eq!(
            self.dimensions, other.dimensions,
            "Vector dimensions must match for dot product"
        );
        
        self.values.iter()
            .zip(other.values.iter())
            .map(|(&a, &b)| a * b)
            .sum()
    }

    /// Compute cosine similarity with another vector
    /// 
    /// Uses precomputed norms for O(1) complexity after dot product.
    /// 
    /// # Arguments
    /// * `other` - Vector to compute similarity with
    /// 
    /// # Returns
    /// * Cosine similarity in range [-1.0, 1.0]
    /// * Returns 0.0 if either vector has zero norm
    pub fn cosine_similarity(&self, other: &Vector) -> f32 {
        if self.norm == 0.0 || other.norm == 0.0 {
            return 0.0;
        }
        
        let dot_product = self.dot(other);
        dot_product / (self.norm * other.norm)
    }

    /// Compute Euclidean distance to another vector
    /// 
    /// # Arguments
    /// * `other` - Vector to compute distance to
    /// 
    /// # Returns
    /// * Euclidean distance (L2 distance)
    pub fn euclidean_distance(&self, other: &Vector) -> f32 {
        assert_eq!(
            self.dimensions, other.dimensions,
            "Vector dimensions must match for distance calculation"
        );
        
        self.values.iter()
            .zip(other.values.iter())
            .map(|(&a, &b)| {
                let diff = a - b;
                diff * diff
            })
            .sum::<f32>()
            .sqrt()
    }

    /// Normalize the vector to unit length
    /// 
    /// After normalization, the norm will be 1.0 (or 0.0 for zero vectors)
    pub fn normalize(&mut self) {
        if self.norm == 0.0 {
            return;
        }
        
        for value in &mut self.values {
            *value /= self.norm;
        }
        
        self.norm = 1.0;
    }

    /// Create a normalized copy of the vector
    pub fn normalized(&self) -> Self {
        let mut copy = self.clone();
        copy.normalize();
        copy
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        if self.dimensions != other.dimensions {
            return false;
        }
        
        // Compare with small epsilon for floating point equality
        const EPSILON: f32 = 1e-6;
        
        self.values.iter()
            .zip(other.values.iter())
            .all(|(&a, &b)| (a - b).abs() < EPSILON)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let values = vec![1.0, 2.0, 3.0];
        let vector = Vector::new(values.clone());
        
        assert_eq!(vector.dimensions, 3);
        assert_eq!(vector.values, values);
        
        // Norm should be sqrt(1^2 + 2^2 + 3^2) = sqrt(14) ≈ 3.742
        let expected_norm = 14.0_f32.sqrt();
        assert!((vector.norm - expected_norm).abs() < 1e-6);
    }

    #[test]
    fn test_zeros() {
        let vector = Vector::zeros(5);
        assert_eq!(vector.dimensions, 5);
        assert_eq!(vector.values, vec![0.0; 5]);
        assert_eq!(vector.norm, 0.0);
    }

    #[test]
    fn test_dot_product() {
        let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
        let v2 = Vector::new(vec![4.0, 5.0, 6.0]);
        
        // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
        let dot = v1.dot(&v2);
        assert!((dot - 32.0).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity() {
        let v1 = Vector::new(vec![1.0, 0.0, 0.0]);
        let v2 = Vector::new(vec![1.0, 0.0, 0.0]);
        
        // Identical vectors should have similarity 1.0
        let sim = v1.cosine_similarity(&v2);
        assert!((sim - 1.0).abs() < 1e-6);
        
        // Orthogonal vectors should have similarity 0.0
        let v3 = Vector::new(vec![0.0, 1.0, 0.0]);
        let sim2 = v1.cosine_similarity(&v3);
        assert!(sim2.abs() < 1e-6);
    }

    #[test]
    fn test_euclidean_distance() {
        let v1 = Vector::new(vec![0.0, 0.0, 0.0]);
        let v2 = Vector::new(vec![3.0, 4.0, 0.0]);
        
        // Distance should be 5.0 (3-4-5 triangle)
        let dist = v1.euclidean_distance(&v2);
        assert!((dist - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_normalize() {
        let mut vector = Vector::new(vec![3.0, 4.0, 0.0]);
        vector.normalize();
        
        // After normalization, norm should be 1.0
        assert!((vector.norm - 1.0).abs() < 1e-6);
        
        // Values should be scaled: [3/5, 4/5, 0]
        assert!((vector.values[0] - 0.6).abs() < 1e-6);
        assert!((vector.values[1] - 0.8).abs() < 1e-6);
        assert!(vector.values[2].abs() < 1e-6);
    }

    #[test]
    fn test_update_norm() {
        let mut vector = Vector::new(vec![1.0, 2.0, 3.0]);
        let original_norm = vector.norm;
        
        // Modify values directly
        vector.values[0] = 2.0;
        
        // Norm should be outdated
        assert_eq!(vector.norm, original_norm);
        
        // Update norm
        vector.update_norm();
        
        // New norm should be sqrt(2^2 + 2^2 + 3^2) = sqrt(17)
        let expected_norm = 17.0_f32.sqrt();
        assert!((vector.norm - expected_norm).abs() < 1e-6);
    }

    #[test]
    #[should_panic(expected = "Vector values cannot be empty")]
    fn test_empty_vector_panics() {
        Vector::new(vec![]);
    }

    #[test]
    fn test_vector_serialization() {
        let vector = Vector::new(vec![1.0, 2.0, 3.0]);
        let serialized = serde_json::to_string(&vector).unwrap();
        let deserialized: Vector = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(vector, deserialized);
    }
}
