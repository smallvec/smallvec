#[cfg(feature = "nightly")]
use core::iter::TrustedLen;
#[cfg(feature = "defmt")]
use defmt::{
    Format as DeFormat,
    Formatter as DeFormatter,
    write as dewrite
};
use {
    super::{
        super::SmallVec,
        drain::Drain
    },
    alloc::alloc::Global,
    core::{
        alloc::Allocator,
        fmt::{
            Debug,
            Formatter,
            Result as Format
        },
        iter::FusedIterator,
        mem::take
    }
};

#[doc = include_str!("../../docs/Splice.md")]
pub struct Splice<
    'valid,
    Iterable: Iterator + 'valid,
    const INLINE: usize,
    Heap: Allocator = Global
> {
    #[doc = include_str!("../../docs/Splice/drain.md")]
    pub drain: Drain<'valid, Iterable::Item, INLINE, Heap>,
    #[doc = include_str!("../../docs/Splice/replacement.md")]
    pub replacement: Iterable
}

impl<Iterable: Iterator, const INLINE: usize, Heap: Allocator> Iterator
    for Splice<'_, Iterable, INLINE, Heap>
{
    type Item = Iterable::Item;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> { self.drain.next() }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) { self.drain.size_hint() }
}

impl<Iterable: Iterator, const INLINE: usize, Heap: Allocator> DoubleEndedIterator
    for Splice<'_, Iterable, INLINE, Heap>
{
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> { self.drain.next_back() }
}

impl<Iterable: Iterator, const INLINE: usize, Heap: Allocator> ExactSizeIterator
    for Splice<'_, Iterable, INLINE, Heap>
{
}

impl<Iterable: Iterator, const INLINE: usize, Heap: Allocator> FusedIterator
    for Splice<'_, Iterable, INLINE, Heap>
{
}

#[cfg(feature = "nightly")]
unsafe impl<Iterable: Iterator, const INLINE: usize, Heap: Allocator> TrustedLen
    for Splice<'_, Iterable, INLINE, Heap>
{
}

impl<Iterable: Iterator, const INLINE: usize, Heap: Allocator> Drop
    for Splice<'_, Iterable, INLINE, Heap>
{
    #[inline]
    fn drop(&mut self) {
        self.drain.by_ref().for_each(drop);
        drop(take(&mut self.drain.iter));
        if self.drain.tail_len == 0 {
            unsafe { self.drain.vec.as_mut() }.extend(&mut self.replacement);
            return;
        }
        if !unsafe { self.drain.fill(&mut self.replacement) } {
            return;
        }
        let bound = self.replacement.size_hint().0;
        if bound > 0 {
            unsafe { self.drain.move_tail(bound) };
            if !unsafe { self.drain.fill(&mut self.replacement) } {
                return;
            }
        }
        let mut smallvec =
            SmallVec::<_, INLINE, _>::new_in(&unsafe { self.drain.vec.as_ref() }.allocator);
        smallvec.extend(&mut self.replacement);
        let mut collected = smallvec.into_iter();
        if collected.len() > 0 {
            unsafe {
                self.drain.move_tail(collected.len());
                self.drain.fill(&mut collected);
            }
        }
    }
}

impl<'valid, Iterable: Iterator<Item: Debug> + 'valid, const INLINE: usize, Heap: Allocator> Debug
    for Splice<'valid, Iterable, INLINE, Heap>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Format {
        f.debug_tuple("Splice").field(&self.drain).finish()
    }
}

#[cfg(feature = "defmt")]
impl<'valid, Iterable: Iterator<Item: DeFormat> + 'valid, const INLINE: usize, Heap: Allocator>
    DeFormat for Splice<'valid, Iterable, INLINE, Heap>
{
    fn format(&self, fmt: DeFormatter<'_>) {
        dewrite!(fmt, "Splice({})", self.drain);
    }
}
