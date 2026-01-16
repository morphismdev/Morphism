use crate::IntoContainer;
use crate::Tagged;

/// Reconstruct a concrete host type from a container representation.
///
/// "Same tag, new payload" (used by `collect`).
pub trait FromContainer: IntoContainer {
    type Rewrap<NewPayload>;

    fn from_container<NewPayload>(payload: NewPayload) -> Self::Rewrap<NewPayload>;
}

impl<K, T> FromContainer for Tagged<K, T> {
    type Rewrap<NewPayload> = Tagged<K, NewPayload>;

    #[inline]
    fn from_container<NewPayload>(payload: NewPayload) -> Self::Rewrap<NewPayload> {
        Tagged::new(payload)
    }
}
