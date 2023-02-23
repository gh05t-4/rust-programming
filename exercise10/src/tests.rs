#[cfg(test)]
mod tests {
    use crate::is_even;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(4));
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(5));
    }
}