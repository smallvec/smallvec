use {
    super::SmallVec,
    alloc::alloc::Global,
    core::alloc::Allocator,
    rkyv::{
        Archive,
        Archived,
        Deserialize,
        Place,
        Serialize,
        rancor::Fallible,
        ser::{
            Allocator as RkyvAllocator,
            Writer
        },
        vec::{
            ArchivedVec,
            VecResolver
        }
    }
};

impl<Item: Archive, const INLINE: usize, Heap: Allocator> Archive for SmallVec<Item, INLINE, Heap> {
    type Archived = ArchivedVec<Archived<Item>>;
    type Resolver = VecResolver;

    #[inline(always)]
    fn resolve(&self, resolver: Self::Resolver, out: Place<Self::Archived>) {
        ArchivedVec::resolve_from_slice(self, resolver, out)
    }
}

impl<
    Item: Serialize<Serializer>,
    Serializer: Fallible + RkyvAllocator + Writer + ?Sized,
    const INLINE: usize,
    Heap: Allocator
> Serialize<Serializer> for SmallVec<Item, INLINE, Heap>
{
    #[inline(always)]
    fn serialize(&self, serializer: &mut Serializer) -> Result<Self::Resolver, Serializer::Error> {
        ArchivedVec::serialize_from_slice(self, serializer)
    }
}

impl<Item: Archive, Deserializer: Fallible + ?Sized, const INLINE: usize>
    Deserialize<SmallVec<Item, INLINE, Global>, Deserializer> for ArchivedVec<Archived<Item>>
where Archived<Item>: Deserialize<Item, Deserializer>
{
    #[inline]
    fn deserialize(
        &self,
        deserializer: &mut Deserializer
    ) -> Result<SmallVec<Item, INLINE, Global>, Deserializer::Error> {
        let mut result = SmallVec::<Item, INLINE, Global>::with_capacity(self.len());
        for item in self.as_slice() {
            result.push(item.deserialize(deserializer)?);
        }
        Ok(result)
    }
}
