#[cfg(feature = "defmt")]
use defmt::{
    Format,
    Formatter as DeFormatter,
    write as dewrite
};
use {
    add_syntax::prepend,
    alloc::alloc::handle_alloc_error,
    core::{
        alloc::Layout,
        error::Error,
        fmt::{
            Debug,
            Display,
            Formatter,
            Result
        }
    }
};

#[derive(Debug, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(not(feature = "nightly"), derive(Clone))]
#[cfg_attr(feature = "nightly", derive_const(Clone))]
#[doc = include_str!("../docs/SmallVecError.md")]
pub enum SmallVecError {
    #[doc = include_str!("../docs/SmallVecError/CapacityOverflow.md")]
    CapacityOverflow,
    #[doc = include_str!("../docs/SmallVecError/AllocationError.md")]
    AllocationError(Layout)
}

impl SmallVecError {
    #[cold]
    #[inline(never)]
    #[track_caller]
    #[doc = include_str!("../docs/SmallVecError/handle.md")]
    #[cfg_attr(feature = "nightly", prepend(const))]
    #[prepend(pub)]
    fn handle<Type>(self) -> Type {
        match self {
            SmallVecError::CapacityOverflow => panic!("capacity overflow"),
            SmallVecError::AllocationError(layout) => handle_alloc_error(layout)
        }
    }
}

impl Error for SmallVecError {}

impl Display for SmallVecError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "Allocation error: {self:?}")
    }
}

#[cfg(feature = "defmt")]
impl Format for SmallVecError {
    fn format(&self, formatter: DeFormatter<'_>) { dewrite!(formatter, "{}", self) }
}
