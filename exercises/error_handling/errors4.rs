#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        if value < 0 {
            Err(CreationError::Negative)
        } else if value == 0 {
            Err(CreationError::Zero)
        } else {
            Ok(PositiveNonzeroInteger(value as u64))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert!(PositiveNonzeroInteger::new(10).is_ok());
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative)
        );
        assert_eq!(
            PositiveNonzeroInteger::new(0),
            Err(CreationError::Zero)
        );
    }
}