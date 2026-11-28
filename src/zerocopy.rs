use {
    super::SmallVec,
    alloc::alloc::Global,
    zerocopy::{
        FromZeros,
        Immutable,
        Maybe,
        TryFromBytes,
        invariant::Alignment
    }
};

unsafe impl<Item, const INLINE: usize> TryFromBytes for SmallVec<Item, INLINE, Global> {
    #[inline(always)]
    fn only_derive_is_allowed_to_implement_this_trait() {}

    #[inline(always)]
    fn is_safe<Align: Alignment>(_candidate: Maybe<'_, Self, Align>) -> bool { false }
}

unsafe impl<Item, const INLINE: usize> FromZeros for SmallVec<Item, INLINE, Global> {
    #[inline(always)]
    fn only_derive_is_allowed_to_implement_this_trait() {}
}

unsafe impl<Item: Immutable, const INLINE: usize> Immutable for SmallVec<Item, INLINE, Global> {
    #[inline(always)]
    fn only_derive_is_allowed_to_implement_this_trait() {}
}
