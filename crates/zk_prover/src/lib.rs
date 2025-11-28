//! Proof generation skeleton modelled after the SP1 workflow.

pub fn version() -> &'static str { env!("CARGO_PKG_VERSION") }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn version_is_set() { assert!(!version().is_empty()); }
}
