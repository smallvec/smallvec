use super::{
    Allocator,
    parts::Parts
};

pub trait Like {
    type Type;
    type Item;
    type Heap: Allocator;
    fn destruct(value: Self::Type) -> Parts<Self::Item, Self::Heap>;
    unsafe fn construct(parts: Parts<Self::Item, Self::Heap>) -> Self::Type;
}
