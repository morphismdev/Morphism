use crate::{U0, U1, U2, U3, U4, U5, U6, U7};

// Domain codes (globally unique). Registry owns these numbers.
// Domain codes are contiguous.
#[allow(non_camel_case_types)]
pub type D_BOOL = U0;
#[allow(non_camel_case_types)]
pub type D_COMBINATORS = U1;
#[allow(non_camel_case_types)]
pub type D_GENERIC = U2;
#[allow(non_camel_case_types)]
pub type D_HLIST = U3;
#[allow(non_camel_case_types)]
pub type D_NEW_TYPE = U4;
#[allow(non_camel_case_types)]
pub type D_OP = U5;
#[allow(non_camel_case_types)]
pub type D_TAGGED = U6;
#[allow(non_camel_case_types)]
pub type D_HLIST_FOLD = U7;

// next free: U8 (you will update this manually as you add domains)
