use {
    super::super::SmallVec,
    alloc::alloc::Global,
    core::{
        fmt::{
            Formatter,
            Result as Format
        },
        marker::PhantomData
    },
    serde_core::{
        Deserialize,
        de::{
            Error,
            SeqAccess,
            Visitor as DeVisitor
        }
    }
};

pub struct Visitor<Item, const INLINE: usize>(pub PhantomData<Item>);

impl<'valid, Item: Deserialize<'valid>, const INLINE: usize> DeVisitor<'valid>
    for Visitor<Item, INLINE>
{
    type Value = SmallVec<Item, INLINE, Global>;

    #[inline(always)]
    fn expecting(&self, formatter: &mut Formatter<'_>) -> Format {
        formatter.write_str("a sequence")
    }

    #[inline]
    fn visit_seq<Sequence: SeqAccess<'valid>>(
        self,
        mut sequence: Sequence
    ) -> Result<Self::Value, Sequence::Error> {
        let mut smallvec = SmallVec::try_with_capacity(sequence.size_hint().unwrap_or(0))
            .map_err(Sequence::Error::custom)?;
        while let Some(value) = sequence.next_element()? {
            smallvec.push(value);
        }
        Ok(smallvec)
    }
}
