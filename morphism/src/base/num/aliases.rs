//! Convenience aliases for small binary naturals (`U0..U64`).
//!
//! - Encoding is the `UTerm/UInt<B, T>` LSB-first nat syntax.
//! - These are purely ergonomic aliases; they introduce no new semantics.
//!
//! See `base/num/README.md` → "Reading guide (syntax + notation)".

use super::nat::{UInt, UTerm, B0, B1};

pub type U0 = UTerm; // 0
pub type U1 = UInt<B1, U0>; // 1
pub type U2 = UInt<B0, U1>; // 2
pub type U3 = UInt<B1, U1>; // 3
pub type U4 = UInt<B0, U2>; // 4
pub type U5 = UInt<B1, U2>; // 5
pub type U6 = UInt<B0, U3>; // 6
pub type U7 = UInt<B1, U3>; // 7
pub type U8 = UInt<B0, U4>; // 8
pub type U9 = UInt<B1, U4>; // 9
pub type U10 = UInt<B0, U5>; // 10
pub type U11 = UInt<B1, U5>; // 11
pub type U12 = UInt<B0, U6>; // 12
pub type U13 = UInt<B1, U6>; // 13
pub type U14 = UInt<B0, U7>; // 14
pub type U15 = UInt<B1, U7>; // 15
pub type U16 = UInt<B0, U8>; // 16
pub type U17 = UInt<B1, U8>; // 17
pub type U18 = UInt<B0, U9>; // 18
pub type U19 = UInt<B1, U9>; // 19
pub type U20 = UInt<B0, U10>; // 20
pub type U21 = UInt<B1, U10>; // 21
pub type U22 = UInt<B0, U11>; // 22
pub type U23 = UInt<B1, U11>; // 23
pub type U24 = UInt<B0, U12>; // 24
pub type U25 = UInt<B1, U12>; // 25
pub type U26 = UInt<B0, U13>; // 26
pub type U27 = UInt<B1, U13>; // 27
pub type U28 = UInt<B0, U14>; // 28
pub type U29 = UInt<B1, U14>; // 29
pub type U30 = UInt<B0, U15>; // 30
pub type U31 = UInt<B1, U15>; // 31
pub type U32 = UInt<B0, U16>; // 32
pub type U33 = UInt<B1, U16>; // 33
pub type U34 = UInt<B0, U17>; // 34
pub type U35 = UInt<B1, U17>; // 35
pub type U36 = UInt<B0, U18>; // 36
pub type U37 = UInt<B1, U18>; // 37
pub type U38 = UInt<B0, U19>; // 38
pub type U39 = UInt<B1, U19>; // 39
pub type U40 = UInt<B0, U20>; // 40
pub type U41 = UInt<B1, U20>; // 41
pub type U42 = UInt<B0, U21>; // 42
pub type U43 = UInt<B1, U21>; // 43
pub type U44 = UInt<B0, U22>; // 44
pub type U45 = UInt<B1, U22>; // 45
pub type U46 = UInt<B0, U23>; // 46
pub type U47 = UInt<B1, U23>; // 47
pub type U48 = UInt<B0, U24>; // 48
pub type U49 = UInt<B1, U24>; // 49
pub type U50 = UInt<B0, U25>; // 50
pub type U51 = UInt<B1, U25>; // 51
pub type U52 = UInt<B0, U26>; // 52
pub type U53 = UInt<B1, U26>; // 53
pub type U54 = UInt<B0, U27>; // 54
pub type U55 = UInt<B1, U27>; // 55
pub type U56 = UInt<B0, U28>; // 56
pub type U57 = UInt<B1, U28>; // 57
pub type U58 = UInt<B0, U29>; // 58
pub type U59 = UInt<B1, U29>; // 59
pub type U60 = UInt<B0, U30>; // 60
pub type U61 = UInt<B1, U30>; // 61
pub type U62 = UInt<B0, U31>; // 62
pub type U63 = UInt<B1, U31>; // 63
pub type U64 = UInt<B0, U32>; // 64
