use {
    super::SmallVec,
    core::alloc::Allocator,
    malloc_size_of::{
        MallocShallowSizeOf,
        MallocSizeOf,
        MallocSizeOfOps
    }
};

impl<Item, const INLINE: usize, Heap: Allocator> MallocShallowSizeOf
    for SmallVec<Item, INLINE, Heap>
{
    #[inline(always)]
    fn shallow_size_of(&self, operations: &mut MallocSizeOfOps) -> usize {
        self.spilled()
            .then(|| unsafe { operations.malloc_size_of(self.as_ptr()) })
            .unwrap_or_default()
    }
}

impl<Item: MallocSizeOf, const INLINE: usize, Heap: Allocator> MallocSizeOf
    for SmallVec<Item, INLINE, Heap>
{
    #[inline(always)]
    fn size_of(&self, operations: &mut MallocSizeOfOps) -> usize {
        self.shallow_size_of(operations)
            + self
                .iter()
                .map(|item| item.size_of(operations))
                .sum::<usize>()
    }
}
