pub mod visitor;

use {
    super::SmallVec,
    alloc::alloc::Global,
    core::{
        alloc::Allocator,
        marker::PhantomData
    },
    serde_core::{
        de::{
            Deserialize,
            Deserializer
        },
        ser::{
            Serialize,
            SerializeSeq,
            Serializer
        }
    },
    visitor::Visitor
};

impl<Item: Serialize, const INLINE: usize, Heap: Allocator> Serialize
    for SmallVec<Item, INLINE, Heap>
{
    #[inline]
    fn serialize<Serializing: Serializer>(
        &self,
        serializer: Serializing
    ) -> Result<Serializing::Ok, Serializing::Error> {
        let mut state = serializer.serialize_seq(Some(self.len()))?;
        for item in self {
            state.serialize_element(item)?;
        }
        state.end()
    }
}

impl<'valid, Item: Deserialize<'valid>, const INLINE: usize> Deserialize<'valid>
    for SmallVec<Item, INLINE, Global>
{
    #[inline(always)]
    fn deserialize<Deserializing: Deserializer<'valid>>(
        deserializer: Deserializing
    ) -> Result<Self, Deserializing::Error> {
        deserializer.deserialize_seq(Visitor(PhantomData))
    }
}
