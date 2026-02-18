/// Apply a single delta to the current state, producing a new state.
///
/// This is the foundational pure function of the entire system:
/// `reduce(state, delta) → new_state`
///
/// Determinism guarantee: identical inputs always produce identical outputs.
pub fn reduce(state: f32, delta: f32) -> f32 {
    state + delta
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduce_applies_delta_to_state() {
        assert_eq!(reduce(0.0, 1.0), 1.0);
    }

    #[test]
    fn reduce_with_negative_delta() {
        assert_eq!(reduce(5.0, -3.0), 2.0);
    }

    #[test]
    fn reduce_with_zero_delta_is_identity() {
        assert_eq!(reduce(42.0, 0.0), 42.0);
    }

    #[test]
    fn reduce_accumulates_multiple_deltas() {
        let state = 0.0;
        let state = reduce(state, 1.0);
        let state = reduce(state, 2.0);
        let state = reduce(state, 3.0);
        assert_eq!(state, 6.0);
    }

    #[test]
    fn reduce_is_deterministic() {
        let result1 = reduce(10.0, 0.5);
        let result2 = reduce(10.0, 0.5);
        assert_eq!(result1, result2);
    }
}
