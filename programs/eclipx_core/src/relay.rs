pub fn calculate_privacy_score(
    relay_count: u8,
    obfuscation_level: u8,
    total_transactions: u64,
) -> u16 {
    let base = (relay_count as u16).saturating_mul(12);
    let obf_bonus = (obfuscation_level as u16).saturating_mul(8);
    let tx_bonus = (total_transactions.min(100) as u16).saturating_mul(2);

    let raw = base.saturating_add(obf_bonus).saturating_add(tx_bonus);
    raw.min(1000)
}

pub fn select_relay_path(seed: &[u8], count: u8) -> Vec<u8> {
    let mut path = Vec::with_capacity(count as usize);
    for i in 0..count {
        let idx = seed.get(i as usize).copied().unwrap_or(i) % 47;
        path.push(idx);
    }
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_privacy_score_basic() {
        let score = calculate_privacy_score(5, 3, 10);
        assert!(score > 0 && score <= 1000);
    }

    #[test]
    fn test_privacy_score_capped() {
        let score = calculate_privacy_score(255, 255, 10000);
        assert_eq!(score, 1000);
    }

    #[test]
    fn test_select_relay_path_length() {
        let seed = [1u8; 32];
        let path = select_relay_path(&seed, 5);
        assert_eq!(path.len(), 5);
    }
}

// touch: bd739ea2

// touch: c06f79e0

// touch: a5907735

// touch: 102f56d6
