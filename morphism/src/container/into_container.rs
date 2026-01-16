use crate::Container;
use crate::{Id, Tagged};

/// Convert a concrete host type into its generic-domain container representation
/// (a `Container<Self, Id>` that encapsulates the tagged representation + an
/// identity program).
pub trait IntoContainer: Sized {
    type Tag;
    type Payload;

    fn into_container(self) -> Container<Self, Id>;
}

impl<K, T> IntoContainer for Tagged<K, T> {
    type Tag = K;
    type Payload = T;

    #[inline]
    fn into_container(self) -> Container<Self, Id> {
        Container::<Self, Id>::from_tagged(self)
    }
}
