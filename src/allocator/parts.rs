use {
    super::Allocator,
    core::{
        mem::MaybeUninit,
        ptr::NonNull
    }
};

#[derive(Debug)]
pub struct Parts<Item, Heap: Allocator> {
    pub pointer: Option<NonNull<[MaybeUninit<Item>]>>,
    pub length: usize,
    pub allocator: Heap
}
