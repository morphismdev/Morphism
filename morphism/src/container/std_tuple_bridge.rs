use crate::{Container, FromContainer, IntoContainer};
use crate::{Id, Tagged, IX_HLISTNODE};

// Tag plain Rust tuples as "prod nodes" by viewing them as `HlistNode<Tuple>`.
macro_rules! impl_tuple_into_from_tagged {
    ( $( $A:ident ),+ $(,)? ) => {
        impl<$( $A, )+> IntoContainer for ( $( $A, )+ ) {
            type Tag = IX_HLISTNODE;
            type Payload = ( $( $A, )+ );

            #[inline]
            fn into_container(self) -> Container<Self, Id> {
                Container::<Self, Id>::from_tagged(Tagged::new(self))
            }
        }

        // Make tuples compatible with `Mappable` blanket impl:
        // `from_container` just strips the tag and returns the payload.
        impl<$( $A, )+> FromContainer for ( $( $A, )+ ) {
            type Rewrap<NewPayload> = NewPayload;

            #[inline]
            fn from_container<NewPayload>(payload: NewPayload) -> Self::Rewrap<NewPayload> {
                payload
            }
        }
    };
}

// Optional: treat `()` as the empty HList.
impl IntoContainer for () {
    type Tag = IX_HLISTNODE;
    type Payload = ();

    #[inline]
    fn into_container(self) -> Container<Self, Id> {
        Container::<Self, Id>::from_tagged(Tagged::new(self))
    }
}

impl FromContainer for () {
    type Rewrap<NewPayload> = NewPayload;

    #[inline]
    fn from_container<NewPayload>(payload: NewPayload) -> Self::Rewrap<NewPayload> {
        payload
    }
}

// 1..=11
impl_tuple_into_from_tagged!(A0);
impl_tuple_into_from_tagged!(A0, A1);
impl_tuple_into_from_tagged!(A0, A1, A2);
impl_tuple_into_from_tagged!(A0, A1, A2, A3);
impl_tuple_into_from_tagged!(A0, A1, A2, A3, A4);
impl_tuple_into_from_tagged!(A0, A1, A2, A3, A4, A5);
impl_tuple_into_from_tagged!(A0, A1, A2, A3, A4, A5, A6);
impl_tuple_into_from_tagged!(A0, A1, A2, A3, A4, A5, A6, A7);
impl_tuple_into_from_tagged!(A0, A1, A2, A3, A4, A5, A6, A7, A8);
impl_tuple_into_from_tagged!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_tuple_into_from_tagged!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);

// 12 (the "tail slot" pattern used elsewhere; last element can itself be a nested tuple)
impl_tuple_into_from_tagged!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, Tail);

#[cfg(test)]
mod tests {
    use crate::{assert_type_eq, Tagged, IX_HLISTNODE};
    use crate::{FromContainer, IntoContainer};

    #[test]
    fn into_container_roundtrips_payload_for_small_tuple() {
        let t = (1u8, 2u16, 3u32, 4u64);

        let out: Tagged<IX_HLISTNODE, _> = t.into_container().run();
        assert_eq!(out.into_inner(), (1u8, 2u16, 3u32, 4u64));
    }

    #[test]
    fn into_container_has_hlist_tag_and_payload_type_arity_15_via_nested_tail() {
        // total "arity 15" encoded as (11 head, Tail) where Tail has 4 elements
        type Tail = (char, f32, f64, u128);
        type In = (
            u8,
            u16,
            u32,
            u64,
            usize,
            i8,
            i16,
            i32,
            i64,
            isize,
            bool,
            Tail,
        );

        type Tag = <In as IntoContainer>::Tag;
        type Payload = <In as IntoContainer>::Payload;

        assert_type_eq::<Tag, IX_HLISTNODE>();
        assert_type_eq::<Payload, In>();
    }

    #[test]
    fn from_container_returns_new_payload_directly() {
        let out: u32 = <(u8,) as FromContainer>::from_container(123u32);
        assert_eq!(out, 123u32);
    }
}
