use core::ops::{
    Bound,
    Range,
    RangeBounds
};

#[inline]
#[track_caller]
pub fn range(bounds: impl RangeBounds<usize>, length: usize) -> Range<usize> {
    let start = match bounds.start_bound() {
        Bound::Included(&start) => start,
        Bound::Excluded(&start) => start.strict_add(1),
        Bound::Unbounded => 0
    };
    let end = match bounds.end_bound() {
        Bound::Included(&end) => end.strict_add(1),
        Bound::Excluded(&end) => end,
        Bound::Unbounded => length
    };
    if start > end {
        panic!("slice index starts at {start} but ends at {end}")
    }
    if end > length {
        panic!("range end index {end} out of range for slice of length {length}")
    }
    Range {
        start,
        end
    }
}
