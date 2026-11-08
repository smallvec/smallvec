use {
    super::Allocator,
    core::{
        ptr::NonNull,
        mem::MaybeUninit
    }
};

#[derive(Debug)]
pub struct Parts<Item, Heap: Allocator> {
    pub pointer: Option<NonNull<[MaybeUninit<Item>]>>,
    pub length: usize,
    pub allocator: Heap
}
