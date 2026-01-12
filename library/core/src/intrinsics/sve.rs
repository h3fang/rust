/// Numerically casts a vector, elementwise.
///
/// `T` and `U` must be scalable vectors with the same number of elements.
///
/// When casting floats to integers, the result is truncated. Out-of-bounds result lead to UB.
/// When casting integers to floats, the result is rounded.
/// Otherwise, truncates or extends the value, maintaining the sign for signed integers.
///
/// # Safety
/// Casting from integer types is always safe.
/// Casting between two float types is also always safe.
///
/// Casting floats to integers truncates, following the same rules as `to_int_unchecked`.
/// Specifically, each element must:
/// * Not be `NaN`
/// * Not be infinite
/// * Be representable in the return type, after truncating off its fractional part
#[rustc_intrinsic]
#[rustc_nounwind]
pub const unsafe fn sve_cast<T, U>(x: T) -> U;

/// Selects elements from a predicate.
///
/// `P` must be a scalable vector predicate.
///
/// `T` must be a scalable vector.
///
/// For each element, if the corresponding bit is set in `p`, select the element from
/// `if_true`.  If the corresponding bit is not set in `p`, select the element from
/// `if_false`.
#[rustc_intrinsic]
#[rustc_nounwind]
pub const unsafe fn sve_select<P, T>(p: P, if_true: T, if_false: T) -> T;
