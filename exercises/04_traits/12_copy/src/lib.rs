// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WrappingU32 {
    value: u32,
}

impl std::ops::Add<Self> for WrappingU32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        WrappingU32 {
            value: self.value.wrapping_add(rhs.value),
        }
    }
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
