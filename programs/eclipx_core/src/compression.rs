#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum TxStatus {
    Pending = 0,
    Routing = 1,
    Compressing = 2,
    Confirmed = 3,
    Failed = 4,
}

pub fn compute_merkle_root(leaves: &[[u8; 32]]) -> [u8; 32] {
    if leaves.is_empty() {
        return [0u8; 32];
    }
    if leaves.len() == 1 {
        return leaves[0];
    }

    let mut current_level = leaves.to_vec();

    while current_level.len() > 1 {
        let mut next_level = Vec::new();
        for chunk in current_level.chunks(2) {
            let left = chunk[0];
            let right = if chunk.len() > 1 { chunk[1] } else { chunk[0] };
            let mut combined = [0u8; 64];
            combined[..32].copy_from_slice(&left);
            combined[32..].copy_from_slice(&right);
            next_level.push(hash_pair(&combined));
        }
        current_level = next_level;
    }

    current_level[0]
}

fn hash_pair(data: &[u8; 64]) -> [u8; 32] {
    use anchor_lang::solana_program::hash::hash;
    let h = hash(data);
    h.to_bytes()
}

pub fn verify_proof_hash(proof: &[u8; 32], expected: &[u8; 32]) -> bool {
    proof == expected
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_root_single() {
        let leaf = [42u8; 32];
        let root = compute_merkle_root(&[leaf]);
        assert_eq!(root, leaf);
    }

    #[test]
    fn test_merkle_root_empty() {
        let root = compute_merkle_root(&[]);
        assert_eq!(root, [0u8; 32]);
    }

    #[test]
    fn test_verify_proof() {
        let proof = [1u8; 32];
        assert!(verify_proof_hash(&proof, &proof));
        assert!(!verify_proof_hash(&proof, &[2u8; 32]));
    }
}

// touch: 0ef36adf

// touch: 04b35a70

// touch: 68b6ad67

// touch: 0960c50d

// touch: dff798ba

// touch: 6c23edea

// touch: a4946e4e

// touch: e99d091f

// touch: 32ad594c

// touch: 85f4dfd9

// touch: ac21a054

// touch: eb1d1547

// touch: 6ce21f48

// touch: 52763a8f

// touch: 56fa1624
