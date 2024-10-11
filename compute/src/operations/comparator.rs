use crate::int::GarbledInt;
use crate::operations::circuits::{build_and_execute_comparator, build_and_execute_equality};
use crate::uint::GarbledUint;
use std::cmp::Ordering;

// Implementing comparison operators for GarbledUint
impl<const N: usize> PartialEq for GarbledUint<N> {
    fn eq(&self, other: &Self) -> bool {
        matches!(build_and_execute_comparator(self, other), Ordering::Equal)
    }
}

// Implementing equality for GarbledUint
impl<const N: usize> Eq for GarbledUint<N> {
    // This is a no-op because the implementation of `Ord` is correct
    // and the implementation of `Eq` is derived from `Ord`.
}

// Implementing comparison operators for GarbledUint
#[allow(clippy::non_canonical_partial_ord_impl)]
impl<const N: usize> PartialOrd for GarbledUint<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(build_and_execute_comparator(self, other))
    }
}

// Implementing comparison operators for GarbledUint
impl<const N: usize> Ord for GarbledUint<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        build_and_execute_comparator(self, other)
    }
}

// Implementing comparison operators for GarbledUint
impl<const N: usize> PartialEq<&GarbledUint<N>> for GarbledUint<N> {
    fn eq(&self, other: &&Self) -> bool {
        !build_and_execute_equality(self, other)
    }
}

// Implementing comparison operators for GarbledUint
impl<const N: usize> PartialOrd<&GarbledUint<N>> for GarbledUint<N> {
    fn partial_cmp(&self, other: &&Self) -> Option<Ordering> {
        Some(build_and_execute_comparator(self, *other))
    }
}

// Implementing comparison operators for GarbledInt
impl<const N: usize> PartialEq for GarbledInt<N> {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            build_and_execute_comparator(&self.into(), &other.into()),
            Ordering::Equal
        )
    }
}

// Implementing equality for GarbledInt
impl<const N: usize> Eq for GarbledInt<N> {
    // This is a no-op because the implementation of `Ord` is correct
    // and the implementation of `Eq` is derived from `Ord`.
}

// Implementing comparison operators for GarbledInt
#[allow(clippy::non_canonical_partial_ord_impl)]
impl<const N: usize> PartialOrd for GarbledInt<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(build_and_execute_comparator(&self.into(), &other.into()))
    }
}

// Implementing comparison operators for GarbledInt
impl<const N: usize> Ord for GarbledInt<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        build_and_execute_comparator(&self.into(), &other.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::int::{GarbledInt128, GarbledInt16, GarbledInt32, GarbledInt64, GarbledInt8};
    use crate::uint::{GarbledUint128, GarbledUint16, GarbledUint32, GarbledUint64, GarbledUint8};

    #[test]
    fn test_uint_equality() {
        let a: GarbledUint8 = 123_u8.into();
        let b: GarbledUint8 = 123_u8.into();
        let c: GarbledUint8 = 124_u8.into();

        assert_eq!(&a, &b);
        assert_ne!(&a, &c);
    }

    #[test]
    fn test_unsigned_comparison() {
        let a: GarbledUint8 = 100_u8.into();
        let b: GarbledUint8 = 150_u8.into();

        assert!(a < b);
        assert!(b > a);
        assert!(a != b);

        let c: GarbledUint8 = 200_u8.into();
        let d: GarbledUint8 = 200_u8.into();

        assert!(c == d);
        assert!(c <= d);
        assert!(c >= d);
    }

    #[test]
    fn test_uint_edge_cases() {
        let zero: GarbledUint8 = 0_u8.into();
        let max: GarbledUint8 = u8::MAX.into();

        assert!(zero < max);
        assert!(max > zero);
        assert!(zero != max);
    }

    #[test]
    fn test_uint_larger_comparison() {
        let a16: GarbledUint16 = 1000_u16.into();
        let b16: GarbledUint16 = 2000_u16.into();
        assert!(a16 < b16);

        let a32: GarbledUint32 = 10000_u32.into();
        let b32: GarbledUint32 = 20000_u32.into();
        assert!(a32 < b32);

        let a64: GarbledUint64 = 10000000000_u64.into();
        let b64: GarbledUint64 = 20000000000_u64.into();
        assert!(a64 < b64);

        let a128: GarbledUint128 = 100000000000000000000_u128.into();
        let b128: GarbledUint128 = 200000000000000000000_u128.into();
        assert!(a128 < b128);
    }

    // test signed integer comparison with different sizes
    #[test]
    #[ignore = "reason: int with negative values not implemented"]
    fn test_int_comparison() {
        let d8: GarbledInt8 = (-100_i8).into();
        let e8: GarbledInt8 = 100_i8.into();

        assert!(d8 < e8);
    }

    #[test]
    fn test_int_larger_comparison() {
        let a16: GarbledInt16 = 1000_i16.into();
        let b16: GarbledInt16 = 2000_i16.into();
        assert!(a16 < b16);

        let a32: GarbledInt32 = 10000_i32.into();
        let b32: GarbledInt32 = 20000_i32.into();
        assert!(a32 < b32);

        let a64: GarbledInt64 = 10000000000_i64.into();
        let b64: GarbledInt64 = 20000000000_i64.into();
        assert!(a64 < b64);

        let a128: GarbledInt128 = 100000000000000000000_i128.into();
        let b128: GarbledInt128 = 200000000000000000000_i128.into();
        assert!(a128 < b128);
    }
}
