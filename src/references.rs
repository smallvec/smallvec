#[cfg(feature = "stable-deref")]
use stable_deref_trait::StableDeref;
use {
    super::SmallVec,
    core::{
        alloc::Allocator,
        borrow::{
            Borrow,
            BorrowMut
        },
        ops::{
            Deref,
            DerefMut
        },
        slice::{
            from_raw_parts,
            from_raw_parts_mut
        }
    }
};
#[cfg(feature = "nightly")]
use {
    add_syntax::prepend,
    core::ops::DerefPure as Pure
};

#[cfg_attr(feature = "nightly", prepend(const))]
impl<Item, const INLINE: usize, Heap: Allocator> AsRef<[Item]> for SmallVec<Item, INLINE, Heap> {
    #[inline]
    fn as_ref(&self) -> &[Item] {
        let (length, spilled) = self.length.parts();
        // SAFETY: all the elements in `..length` are initialized
        unsafe { from_raw_parts(self.raw.as_ptr(spilled), length) }
    }
}
#[cfg_attr(feature = "nightly", prepend(const))]
impl<Item, const INLINE: usize, Heap: Allocator> AsMut<[Item]> for SmallVec<Item, INLINE, Heap> {
    #[inline]
    fn as_mut(&mut self) -> &mut [Item] {
        let (length, spilled) = self.length.parts();
        // SAFETY: see above
        unsafe { from_raw_parts_mut(self.raw.as_mut_ptr(spilled), length) }
    }
}

#[cfg_attr(feature = "nightly", prepend(const))]
impl<Item, const INLINE: usize, Heap: Allocator> Borrow<[Item]> for SmallVec<Item, INLINE, Heap> {
    #[inline(always)]
    fn borrow(&self) -> &[Item] { self.as_ref() }
}
#[cfg_attr(feature = "nightly", prepend(const))]
impl<Item, const INLINE: usize, Heap: Allocator> BorrowMut<[Item]>
    for SmallVec<Item, INLINE, Heap>
{
    #[inline(always)]
    fn borrow_mut(&mut self) -> &mut [Item] { self.as_mut() }
}

#[cfg_attr(feature = "nightly", prepend(const))]
impl<Item, const INLINE: usize, Heap: Allocator> Deref for SmallVec<Item, INLINE, Heap> {
    type Target = [Item];

    #[inline(always)]
    fn deref(&self) -> &Self::Target { self.as_ref() }
}
#[cfg_attr(feature = "nightly", prepend(const))]
impl<Item, const INLINE: usize, Heap: Allocator> DerefMut for SmallVec<Item, INLINE, Heap> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target { self.as_mut() }
}

#[cfg(feature = "nightly")]
unsafe impl<Item, const INLINE: usize, Heap: Allocator> Pure for SmallVec<Item, INLINE, Heap> {}

#[cfg(feature = "stable-deref")]
unsafe impl<Item, Heap: Allocator> StableDeref for SmallVec<Item, 0, Heap> {}
