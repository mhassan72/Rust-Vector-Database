# Testing Guidelines for Phenix-DB

Comprehensive testing is critical for Phenix-DB's mathematical correctness and reliability.

## Testing Philosophy

1. **Mathematical Correctness First**: Every mathematical operation must have correctness tests
2. **Property-Based Testing**: Use proptest for mathematical properties
3. **Integration Over Mocking**: Test real functionality, not mocks
4. **Performance Validation**: Benchmark critical paths
5. **Chaos Engineering**: Test failure scenarios

## Test Organization

### Directory Structure

```
phenix-db/
├── src/
│   └── */
│       └── *.rs          # Unit tests in same file
├── tests/
│   ├── integration/      # End-to-end integration tests
│   ├── mathematical/     # Mathematical correctness tests
│   ├── performance/      # Performance benchmarks
│   ├── cognitive/        # Cognitive behavior tests
│   ├── security/         # Security tests
│   └── chaos/           # Chaos engineering tests
└── benches/             # Criterion benchmarks
```

## Unit Tests

### Location

Place unit tests in the same file as the code:

```rust
// src/core/vector.rs

impl Vector {
    pub fn cosine_similarity(&self, other: &Vector) -> f32 {
        // implementation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity() {
        let v1 = Vector::new(vec![1.0, 0.0, 0.0]);
        let v2 = Vector::new(vec![1.0, 0.0, 0.0]);
        
        assert!((v1.cosine_similarity(&v2) - 1.0).abs() < 0.0001);
    }
}
```

### Running Unit Tests

```bash
# Run all unit tests
cargo test

# Run tests for specific module
cargo test core::vector

# Run with output
cargo test -- --nocapture

# Run single test
cargo test test_cosine_similarity
```

## Mathematical Correctness Tests

### Polynomial Evaluation

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_polynomial_evaluation_accuracy() {
        let poly = PolynomialEmbedding {
            coefficients: vec![1.0, 2.0, 3.0], // 1 + 2x + 3x²
            degree: 2,
            // ...
        };

        // Test at multiple points
        assert_relative_eq!(poly.evaluate(0.0), 1.0, epsilon = 0.00001);
        assert_relative_eq!(poly.evaluate(1.0), 6.0, epsilon = 0.00001);
        assert_relative_eq!(poly.evaluate(2.0), 17.0, epsilon = 0.00001);
    }

    #[test]
    fn test_polynomial_recursive_matches_direct() {
        let poly = PolynomialEmbedding::new(vec![1.0, 2.0, 3.0, 4.0]);
        
        for x in [0.0, 0.5, 1.0, 2.0, 10.0] {
            let direct = poly.evaluate(x);
            let recursive = poly.evaluate_recursive(x);
            assert_relative_eq!(direct, recursive, epsilon = 0.00001);
        }
    }
}
```

### Probability Distribution Tests

```rust
#[test]
fn test_probability_sum_invariant() {
    let mut probs = vec![0.2, 0.3, 0.5];
    normalize_probabilities(&mut probs);
    
    let sum: f32 = probs.iter().sum();
    assert!((sum - 1.0).abs() < 0.001, "Probability sum must equal 1.0");
}

#[test]
fn test_probability_bounds() {
    let mut edge = ProbabilisticEdge::new(/* ... */);
    
    // Update many times
    for _ in 0..1000 {
        edge.update_weight(0.1, true);
    }
    
    // Probability must stay in [0, 1]
    assert!(edge.probability >= 0.0);
    assert!(edge.probability <= 1.0);
}
```

### Entropy Tests

```rust
#[test]
fn test_entropy_bounds() {
    let probs = vec![0.25, 0.25, 0.25, 0.25];
    let entropy = shannon_entropy(&probs);
    
    // Entropy must be in [0, log2(n)]
    assert!(entropy >= 0.0);
    assert!(entropy <= 2.0); // log2(4) = 2
}

#[test]
fn test_uniform_distribution_max_entropy() {
    let n = 8;
    let uniform = vec![1.0 / n as f32; n];
    let entropy = shannon_entropy(&uniform);
    let max_entropy = (n as f64).log2();
    
    assert_relative_eq!(entropy, max_entropy, epsilon = 0.001);
}
```

## Property-Based Testing

Use `proptest` for mathematical properties:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_vector_norm_positive(values in prop::collection::vec(-1000.0f32..1000.0, 1..100)) {
        let vector = Vector::new(values);
        prop_assert!(vector.norm >= 0.0);
    }

    #[test]
    fn test_cosine_similarity_bounds(
        v1 in prop::collection::vec(-100.0f32..100.0, 10),
        v2 in prop::collection::vec(-100.0f32..100.0, 10)
    ) {
        let vec1 = Vector::new(v1);
        let vec2 = Vector::new(v2);
        let sim = vec1.cosine_similarity(&vec2);
        
        prop_assert!(sim >= -1.0 && sim <= 1.0);
    }

    #[test]
    fn test_compression_lossless(data in prop::collection::vec(0u8..255, 100..1000)) {
        let compressed = compress(&data)?;
        let decompressed = decompress(&compressed)?;
        prop_assert_eq!(data, decompressed);
    }
}
```

## Integration Tests

### End-to-End Workflow Tests

```rust
// tests/integration/entity_lifecycle.rs

#[tokio::test]
async fn test_entity_insert_retrieve_workflow() {
    // Setup
    let db = MemorySubstrate::new(Config::default()).await?;
    
    // Create entity
    let entity = Entity::new(EntityId::new())
        .with_vector(Vector::new(vec![1.0, 2.0, 3.0]))
        .with_metadata(json!({"name": "test"}));
    
    // Insert
    db.insert(entity.clone()).await?;
    
    // Retrieve
    let retrieved = db.get(entity.id).await?;
    
    // Verify
    assert_eq!(retrieved.id, entity.id);
    assert_eq!(retrieved.vector, entity.vector);
    assert_eq!(retrieved.metadata, entity.metadata);
}

#[tokio::test]
async fn test_vector_search_with_graph_traversal() {
    let db = setup_test_db().await?;
    
    // Insert entities with relationships
    let entities = create_test_entities(100);
    for entity in &entities {
        db.insert(entity.clone()).await?;
    }
    
    // Create edges
    create_test_edges(&db, &entities).await?;
    
    // Query: vector search + graph traversal
    let query = CognitiveQuery {
        query_type: QueryType::Hybrid,
        vector_query: Some(VectorQuery {
            vector: Vector::new(vec![1.0, 0.0, 0.0]),
            k: 10,
            distance_metric: DistanceMetric::Cosine,
            tier_preference: None,
        }),
        graph_traversal: Some(GraphTraversal {
            start_entities: vec![],
            max_depth: 2,
            edge_filter: None,
            use_probabilistic_weights: true,
        }),
        options: QueryOptions::default(),
    };
    
    let results = db.query(query).await?;
    
    // Verify results
    assert!(results.len() <= 10);
    assert!(results.iter().all(|r| r.score > 0.0));
}
```

### Multi-Component Integration

```rust
// tests/integration/rpi_pgm_integration.rs

#[tokio::test]
async fn test_rpi_pgm_integration() {
    let rpi = RecursivePolynomialIndex::new();
    let pgm = ProbabilisticGraphMemory::new();
    
    // Insert entities into RPI
    let entities = create_test_entities(1000);
    for entity in &entities {
        rpi.insert(entity).await?;
    }
    
    // Build graph in PGM based on co-access
    simulate_access_patterns(&rpi, &pgm, &entities).await?;
    
    // Verify polynomial embeddings exist
    for entity in &entities {
        let embedding = rpi.get_embedding(entity.id).await?;
        assert!(embedding.is_some());
    }
    
    // Verify graph edges were created
    let edge_count = pgm.edge_count().await?;
    assert!(edge_count > 0);
    
    // Verify probability distributions
    for entity in entities.iter().take(10) {
        let edges = pgm.get_edges(entity.id).await?;
        let prob_sum: f32 = edges.iter().map(|e| e.probability).sum();
        assert!((prob_sum - 1.0).abs() < 0.001);
    }
}
```

## Performance Tests

### Benchmarks with Criterion

```rust
// benches/polynomial_operations.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use phenix_db::mathematical::polynomial::PolynomialEmbedding;

fn polynomial_evaluation_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("polynomial_evaluation");
    
    for degree in [3, 5, 10, 20] {
        let coeffs = vec![1.0; degree];
        let poly = PolynomialEmbedding::new(coeffs);
        
        group.bench_with_input(
            BenchmarkId::from_parameter(degree),
            &poly,
            |b, poly| {
                b.iter(|| poly.evaluate(black_box(2.0)))
            },
        );
    }
    
    group.finish();
}

fn polynomial_recursive_benchmark(c: &mut Criterion) {
    let poly = PolynomialEmbedding::new(vec![1.0; 10]);
    
    c.bench_function("polynomial_recursive", |b| {
        b.iter(|| poly.evaluate_recursive(black_box(2.0)))
    });
}

criterion_group!(benches, polynomial_evaluation_benchmark, polynomial_recursive_benchmark);
criterion_main!(benches);
```

### Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench polynomial_evaluation

# Save baseline
cargo bench -- --save-baseline main

# Compare against baseline
cargo bench -- --baseline main
```

### Performance Assertions

```rust
#[test]
fn test_hot_tier_latency() {
    let db = setup_test_db();
    let entity = create_test_entity();
    
    db.insert(entity.clone()).await?;
    
    // Measure latency
    let start = Instant::now();
    let _ = db.get(entity.id).await?;
    let latency = start.elapsed();
    
    // Assert P99 < 1ms for hot tier
    assert!(latency < Duration::from_millis(1), 
            "Hot tier latency exceeded 1ms: {:?}", latency);
}
```

## Chaos Engineering Tests

### Failure Injection

```rust
// tests/chaos/node_failures.rs

#[tokio::test]
async fn test_random_node_failures() {
    let cluster = setup_test_cluster(10).await?;
    
    // Start workload
    let workload = spawn_continuous_workload(&cluster);
    
    // Randomly kill nodes
    for _ in 0..5 {
        let node_id = random_node(&cluster);
        cluster.kill_node(node_id).await?;
        
        tokio::time::sleep(Duration::from_secs(1)).await;
        
        // Verify cluster still operational
        assert!(cluster.is_healthy().await?);
    }
    
    // Stop workload and verify no data loss
    let results = workload.await?;
    assert_eq!(results.errors, 0);
    assert_eq!(results.data_loss, 0);
}

#[tokio::test]
async fn test_network_partition() {
    let cluster = setup_test_cluster(6).await?;
    
    // Create partition: [1,2,3] | [4,5,6]
    cluster.partition_network(vec![1,2,3], vec![4,5,6]).await?;
    
    // Both partitions should continue operating
    assert!(cluster.partition_a_healthy().await?);
    assert!(cluster.partition_b_healthy().await?);
    
    // Heal partition
    cluster.heal_partition().await?;
    
    // Verify consistency after healing
    let consistency = cluster.check_consistency().await?;
    assert!(consistency.is_consistent());
}
```

## Test Coverage

### Measuring Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage

# View report
open coverage/index.html
```

### Coverage Goals

- **Core modules**: 90%+ coverage
- **Mathematical operations**: 95%+ coverage
- **Critical paths**: 100% coverage
- **Error handling**: 80%+ coverage

## Continuous Integration

### GitHub Actions Workflow

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run tests
        run: cargo test --all-features
      
      - name: Run clippy
        run: cargo clippy -- -D warnings
      
      - name: Check formatting
        run: cargo fmt -- --check
      
      - name: Run benchmarks
        run: cargo bench --no-run
```

## Test Data Management

### Test Fixtures

```rust
// tests/common/fixtures.rs

pub fn create_test_entity() -> Entity {
    Entity::new(EntityId::new())
        .with_vector(Vector::new(vec![1.0, 2.0, 3.0]))
        .with_metadata(json!({"test": true}))
}

pub fn create_test_entities(count: usize) -> Vec<Entity> {
    (0..count).map(|i| {
        Entity::new(EntityId::new())
            .with_vector(Vector::new(vec![i as f32, 0.0, 0.0]))
            .with_metadata(json!({"index": i}))
    }).collect()
}

pub async fn setup_test_db() -> Result<MemorySubstrate> {
    let config = Config {
        polynomial_index: PolynomialConfig {
            degree: 3,
            ..Default::default()
        },
        ..Default::default()
    };
    
    MemorySubstrate::new(config).await
}
```

### Cleanup

```rust
#[tokio::test]
async fn test_with_cleanup() {
    let temp_dir = tempfile::tempdir()?;
    let db = setup_db_with_path(temp_dir.path()).await?;
    
    // Test operations
    // ...
    
    // Cleanup happens automatically when temp_dir is dropped
}
```

## Best Practices

### DO

✅ Test mathematical properties, not implementation details  
✅ Use property-based testing for mathematical operations  
✅ Test error conditions and edge cases  
✅ Verify performance characteristics  
✅ Test concurrent access patterns  
✅ Use meaningful test names  
✅ Keep tests focused and independent  

### DON'T

❌ Mock mathematical operations  
❌ Test private implementation details  
❌ Create flaky tests with timing dependencies  
❌ Skip cleanup in tests  
❌ Ignore test warnings  
❌ Write tests that depend on execution order  

## Debugging Tests

```bash
# Run test with backtrace
RUST_BACKTRACE=1 cargo test test_name

# Run test with logging
RUST_LOG=debug cargo test test_name -- --nocapture

# Run test in debugger (VS Code)
# Set breakpoint and use "Debug Test" in rust-analyzer
```

## Next Steps

- Review [Code Organization](./code-organization.md)
- Check [Performance Optimization](./performance.md)
- See [Mathematical Correctness](../architecture/mathematical-foundation.md)
- Explore [Implementation Tasks](../../.kiro/specs/phenix-vision-refactor/tasks.md)
