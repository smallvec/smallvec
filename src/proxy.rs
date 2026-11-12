use core::{
    alloc::{
        AllocError,
        Allocator,
        Layout
    },
    ptr::NonNull
};

#[repr(transparent)]
pub struct Proxy<'valid, Heap: Allocator>(pub &'valid Heap);

unsafe impl<Heap: Allocator> Allocator for Proxy<'_, Heap> {
    #[inline]
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        self.0.allocate(layout)
    }

    #[inline]
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        unsafe {
            self.0.deallocate(ptr, layout);
        }
    }

    #[inline]
    unsafe fn grow(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout
    ) -> Result<NonNull<[u8]>, AllocError> {
        unsafe { self.0.grow(ptr, old_layout, new_layout) }
    }

    #[inline]
    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout
    ) -> Result<NonNull<[u8]>, AllocError> {
        unsafe { self.0.shrink(ptr, old_layout, new_layout) }
    }
}
