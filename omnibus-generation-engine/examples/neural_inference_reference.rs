//! Reference Implementation: neural-inference-network-sdk
//!
//! Shows a 4-stage pipeline with KV-cache sharding across 2 nodes.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// One shard of a transformer layer stack.
pub struct ModelShard {
    shard_id: u64,
    layers: Vec<Layer>,
}

struct Layer {
    weights: Vec<f32>,
}

impl ModelShard {
    pub fn new(shard_id: u64, layer_count: usize) -> Self {
        Self {
            shard_id,
            layers: (0..layer_count).map(|_| Layer { weights: vec![0.0; 768] }).collect(),
        }
    }

    pub fn forward(&self, input: &[f32]) -> Vec<f32> {
        // Stub: identity pass
        input.to_vec()
    }
}

/// Routes tokens through a pipeline of shards.
pub struct PipelineRouter {
    stages: Vec<Arc<ModelShard>>,
}

impl PipelineRouter {
    pub fn new(stages: Vec<Arc<ModelShard>>) -> Self {
        Self { stages }
    }

    pub fn run(&self, mut tokens: Vec<f32>) -> Vec<f32> {
        for (i, shard) in self.stages.iter().enumerate() {
            tokens = shard.forward(&tokens);
            eprintln!("Stage {} (shard {}) complete", i, shard.shard_id);
        }
        tokens
    }
}

/// Manages KV-cache entries keyed by request ID and head.
pub struct KVCacheManager {
    cache: Mutex<HashMap<(String, u32), Vec<f32>>>,
    max_len: usize,
}

impl KVCacheManager {
    pub fn new(max_len: usize) -> Self {
        Self {
            cache: Mutex::new(HashMap::new()),
            max_len,
        }
    }

    pub fn get(&self, req_id: &str, head: u32) -> Option<Vec<f32>> {
        self.cache.lock().unwrap().get(&(req_id.to_string(), head)).cloned()
    }

    pub fn append(&self, req_id: &str, head: u32, values: &[f32]) {
        let mut c = self.cache.lock().unwrap();
        let entry = c.entry((req_id.to_string(), head)).or_default();
        entry.extend_from_slice(values);
        if entry.len() > self.max_len {
            entry.drain(0..entry.len() - self.max_len);
        }
    }
}

fn main() {
    let shards: Vec<Arc<ModelShard>> = (0..4)
        .map(|i| Arc::new(ModelShard::new(i, 6)))
        .collect();
    let router = PipelineRouter::new(shards);
    let kv = KVCacheManager::new(2048);

    let input = vec![1.0f32; 768];
    let output = router.run(input);
    kv.append("req-1", 0, &output);
    println!("Inference complete. Output len: {}", output.len());
}
