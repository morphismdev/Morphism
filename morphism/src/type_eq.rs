//! Utility types and operations

/// Type equality trait
pub trait TypeEq<Rhs> {
    /// Coerce `Self` into `Rhs` at zero cost
    fn coerce_right(self) -> Rhs;
    fn coerce_left(t: Rhs) -> Self;
    fn witness();
}

impl<T> TypeEq<T> for T {
    #[inline]
    fn coerce_right(self) -> T {
        self
    }

    #[inline]
    fn coerce_left(t: T) -> Self {
        t
    }

    #[inline]
    fn witness() {}
}

/// Type-level equality assertion.
///
/// Prefer this in internal type-level harnesses (e.g. `laws/*_ty.rs`) to keep the
/// "`TypeEq` witness" boilerplate out of the call site.
#[allow(dead_code)]
#[inline]
pub fn assert_type_eq<A, B>()
where
    A: TypeEq<B>,
{
    <A as TypeEq<B>>::witness();
}
