#[cfg(test)]
mod test_lerp {
    use crate::lerp::*;

    #[test]
    fn test_lerp() {
        let a = 0.0;
        let b = 10.0;
        let t = 0.5;
        let result = t.lerp_unclamped(a, b);
        assert_eq!(result, 5.0);
    }
}
