use {
    super::SmallVec,
    alloc::{
        alloc::{
            Allocator,
            Global
        },
        collections::BTreeMap as Map,
        format
    },
    borsh::{
        BorshDeserialize as Deserialize,
        BorshSchema as Schema,
        BorshSerialize as Serialize,
        io::{
            Error,
            ErrorKind,
            Read,
            Result as Serial,
            Write
        },
        schema::{
            Declaration,
            Definition
        }
    },
    core::iter::repeat_with
};

impl<Item: Serialize, const INLINE: usize, Heap: Allocator> Serialize
    for SmallVec<Item, INLINE, Heap>
{
    #[inline]
    fn serialize<Writer: Write>(&self, writer: &mut Writer) -> Serial<()> {
        (self.len() as u64).serialize(writer)?;
        for element in self {
            element.serialize(writer)?;
        }
        Ok(())
    }
}

impl<Item: Deserialize, const INLINE: usize> Deserialize for SmallVec<Item, INLINE, Global> {
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Serial<Self> {
        let length = u64::deserialize_reader(reader)?;
        repeat_with(|| Item::deserialize_reader(reader))
            .take(length.try_into().map_err(|_| Error::new(
                ErrorKind::OutOfMemory,
                "Cannot deserialize a sequence with more than usize::MAX elements in this machine"
            ))?)
            .collect()
    }
}

impl<Item: Schema, const INLINE: usize, Heap: Allocator> Schema for SmallVec<Item, INLINE, Heap> {
    #[inline(always)]
    fn declaration() -> Declaration { format!("Vec<{}>", Item::declaration()) }

    #[inline]
    fn add_definitions_recursively(definitions: &mut Map<Declaration, Definition>) {
        definitions
            .entry(Self::declaration())
            .or_insert_with(|| Definition::Sequence {
                length_width: 8,
                length_range: 0..=u64::MAX,
                elements: Item::declaration()
            });
        Item::add_definitions_recursively(definitions);
    }
}
