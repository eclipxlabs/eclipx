pub fn interpolate_score(points: &[(f64, f64)], x: f64) -> f64 {
    if points.is_empty() {
        return 0.0;
    }

    let mut result = 0.0;
    let n = points.len();

    for i in 0..n {
        let mut term = points[i].1;
        for j in 0..n {
            if i != j {
                let denom = points[i].0 - points[j].0;
                if denom.abs() > f64::EPSILON {
                    term *= (x - points[j].0) / denom;
                }
            }
        }
        result += term;
    }

    result.clamp(0.0, 1000.0)
}

pub fn weighted_random_selection(weights: &[u32], seed: u64) -> usize {
    let total: u64 = weights.iter().map(|w| *w as u64).sum();
    if total == 0 {
        return 0;
    }

    let target = seed % total;
    let mut cumulative: u64 = 0;

    for (i, w) in weights.iter().enumerate() {
        cumulative += *w as u64;
        if target < cumulative {
            return i;
        }
    }

    weights.len().saturating_sub(1)
}

pub fn compute_latency_estimate(hop_count: u8, base_latency_ms: f64) -> f64 {
    let per_hop = base_latency_ms * 1.15_f64.powi(hop_count as i32);
    per_hop + (hop_count as f64) * 2.8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolate_basic() {
        let points = vec![(0.0, 0.0), (10.0, 100.0)];
        let result = interpolate_score(&points, 5.0);
        assert!((result - 50.0).abs() < 0.001);
    }

    #[test]
    fn test_weighted_selection() {
        let weights = vec![10, 20, 70];
        let idx = weighted_random_selection(&weights, 5);
        assert!(idx < weights.len());
    }

    #[test]
    fn test_latency_estimate() {
        let latency = compute_latency_estimate(3, 10.0);
        assert!(latency > 10.0);
    }
}

// touch: 2582223f

// touch: bf946532

// touch: 18bb2c45

// touch: 53b124a6

// touch: 54e9454a

// touch: 800e5b7d

// touch: c4712490

// touch: 1d27307e

// touch: 4ff8e98e

// touch: c7dc27ea

// touch: d62338ff

// touch: 1a21fa03

// touch: 89e11c57

// touch: d5e35901

// touch: 5a3de3e9
