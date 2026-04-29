use std::time::{Instant};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TauMeasurement {
    pub tool_name: String,
    pub operation_type: String,
    pub execution_time_ms: f64,
    pub success: bool,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TauStatistics {
    pub tool_name: String,
    pub operation_type: String,
    pub count: usize,
    pub avg_ms: f64,
    pub min_ms: f64,
    pub max_ms: f64,
    pub p50_ms: f64,
    pub p95_ms: f64,
    pub p99_ms: f64,
    pub success_rate: f64,
}

pub struct TauTelemetryCollector {
    measurements: RwLock<Vec<TauMeasurement>>,
    max_measurements: usize,
}

impl TauTelemetryCollector {
    pub fn new(max_measurements: usize) -> Self {
        Self {
            measurements: RwLock::new(Vec::with_capacity(max_measurements)),
            max_measurements,
        }
    }

    pub async fn record_measurement(&self, measurement: TauMeasurement) {
        let mut measurements = self.measurements.write().await;
        measurements.push(measurement);
        
        // Keep only the most recent measurements
        if measurements.len() > self.max_measurements {
            let excess = measurements.len() - self.max_measurements;
            measurements.drain(0..excess);
        }
    }

    pub async fn measure_operation<F, T>(&self, tool_name: &str, operation_type: &str, operation: F) -> T
    where
        F: FnOnce() -> T,
    {
        let start = Instant::now();
        let result = operation();
        let duration = start.elapsed();
        
        let measurement = TauMeasurement {
            tool_name: tool_name.to_string(),
            operation_type: operation_type.to_string(),
            execution_time_ms: duration.as_millis() as f64,
            success: true, // Assuming success for now
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
        };

        // Record asynchronously
        let collector = self.clone();
        let measurement_clone = measurement.clone();
        tokio::spawn(async move {
            collector.record_measurement(measurement_clone).await;
        });

        result
    }

    pub async fn measure_async_operation<F, Fut, T>(&self, tool_name: &str, operation_type: &str, operation: F) -> T
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        let start = Instant::now();
        let result = operation().await;
        let duration = start.elapsed();
        
        let measurement = TauMeasurement {
            tool_name: tool_name.to_string(),
            operation_type: operation_type.to_string(),
            execution_time_ms: duration.as_millis() as f64,
            success: true,
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
        };

        // Record asynchronously
        let collector = self.clone();
        let measurement_clone = measurement.clone();
        tokio::spawn(async move {
            collector.record_measurement(measurement_clone).await;
        });

        result
    }

    pub async fn get_statistics(&self) -> Vec<TauStatistics> {
        let measurements = self.measurements.read().await;
        let mut stats_by_tool_op: HashMap<(String, String), Vec<f64>> = HashMap::new();
        let mut success_counts: HashMap<(String, String), (usize, usize)> = HashMap::new();

        for measurement in measurements.iter() {
            let key = (measurement.tool_name.clone(), measurement.operation_type.clone());
            stats_by_tool_op.entry(key.clone())
                .or_insert_with(Vec::new)
                .push(measurement.execution_time_ms);
            
            let (success, total) = success_counts.entry(key).or_insert((0, 0));
            *total += 1;
            if measurement.success {
                *success += 1;
            }
        }

        let mut statistics = Vec::new();
        for ((tool_name, operation_type), times) in stats_by_tool_op {
            let mut sorted_times = times.clone();
            sorted_times.sort_by(|a, b| a.partial_cmp(b).unwrap());
            
            let count = sorted_times.len();
            let sum: f64 = sorted_times.iter().sum();
            let avg = sum / count as f64;
            let min = *sorted_times.first().unwrap_or(&0.0);
            let max = *sorted_times.last().unwrap_or(&0.0);
            
            // Calculate percentiles
            let p50_idx = (count as f64 * 0.5).floor() as usize;
            let p95_idx = (count as f64 * 0.95).floor() as usize;
            let p99_idx = (count as f64 * 0.99).floor() as usize;
            
            let p50 = sorted_times.get(p50_idx).copied().unwrap_or(0.0);
            let p95 = sorted_times.get(p95_idx).copied().unwrap_or(0.0);
            let p99 = sorted_times.get(p99_idx).copied().unwrap_or(0.0);
            
            let (success_count, total_count) = success_counts.get(&(tool_name.clone(), operation_type.clone()))
                .copied()
                .unwrap_or((0, 0));
            let success_rate = if total_count > 0 { success_count as f64 / total_count as f64 } else { 0.0 };

            statistics.push(TauStatistics {
                tool_name,
                operation_type,
                count,
                avg_ms: avg,
                min_ms: min,
                max_ms: max,
                p50_ms: p50,
                p95_ms: p95,
                p99_ms: p99,
                success_rate,
            });
        }

        statistics
    }

    pub async fn clear_measurements(&self) {
        let mut measurements = self.measurements.write().await;
        measurements.clear();
    }

    pub async fn get_measurements(&self) -> Vec<TauMeasurement> {
        self.measurements.read().await.clone()
    }
}

impl Clone for TauTelemetryCollector {
    fn clone(&self) -> Self {
        Self {
            measurements: RwLock::new(Vec::new()), // Start with empty measurements for clones
            max_measurements: self.max_measurements,
        }
    }
}
