use {
    smallvec::SmallVec,
    std::{
        alloc::Global,
        rc::Rc
    }
};

#[test]
fn into_from_parts_spilled() {
    let v: SmallVec<i32, 1> = SmallVec::from([-1, 0, 1]);
    assert!(v.spilled());

    let (length, raw) = v.into();
    assert!(length.is_spilled());
    assert_eq!(length.len(), 3);

    // SAFETY: `length` says spilled, so `heap` is the active member.
    let pointer = unsafe { raw.heap };
    assert!(pointer.len() > 1);

    // SAFETY: parts come straight from `into_parts`.
    let rebuilt = unsafe { SmallVec::<i32, 1>::parts(length, raw) };
    assert_eq!(rebuilt, [-1, 0, 1]);
    assert_eq!(rebuilt.as_ptr(), pointer.as_ptr() as *const i32);
}

#[test]
fn into_from_parts_inline() {
    let v: SmallVec<i32, 4> = SmallVec::from([1, 2, 3]);
    assert!(!v.spilled());

    let (length, raw) = v.into();
    assert!(!length.is_spilled());
    assert_eq!(length.len(), 3);

    // SAFETY: parts come straight from `into_parts`.
    let rebuilt = unsafe { SmallVec::<i32, 4>::parts(length, raw) };
    assert_eq!(rebuilt, [1, 2, 3]);
}

#[test]
fn parts_do_not_drop_or_double_drop() {
    let rc = Rc::new(());

    for spilled in [false, true] {
        // INLINE = 2: two items stay inline, three spill
        let count = if spilled { 3 } else { 2 };
        let mut v: SmallVec<Rc<()>, 2> = SmallVec::new();
        for _ in 0..count {
            v.push(rc.clone());
        }
        assert_eq!(v.spilled(), spilled);
        assert_eq!(Rc::strong_count(&rc), 1 + count);

        let (length, raw) = v.into();
        // nothing was dropped by `into_parts`
        assert_eq!(Rc::strong_count(&rc), 1 + count);

        // SAFETY: parts come straight from `into_parts`.
        let rebuilt = unsafe { SmallVec::<Rc<()>, 2>::parts(length, raw) };
        assert_eq!(Rc::strong_count(&rc), 1 + count);

        drop(rebuilt);
        // dropped exactly once
        assert_eq!(Rc::strong_count(&rc), 1);
    }
}

#[test]
fn into_from_parts_empty() {
    let v: SmallVec<i32, 2> = SmallVec::new();

    let (length, raw) = v.into();
    assert_eq!(length.len(), 0);
    assert!(!length.is_spilled());

    // SAFETY: parts come straight from `into_parts`.
    let rebuilt = unsafe { SmallVec::<i32, 2>::parts(length, raw) };
    assert!(rebuilt.is_empty());
}

#[test]
fn into_from_parts_zst() {
    let v: SmallVec<(), 2> = SmallVec::from([(), (), ()]);

    let (length, raw) = v.into();
    assert!(!length.is_spilled());
    assert_eq!(length.len(), 3);

    // SAFETY: parts come straight from `into_parts`.
    let rebuilt = unsafe { SmallVec::<(), 2>::parts(length, raw) };
    assert_eq!(rebuilt.len(), 3);
}

#[test]
fn round_trip_with_allocator() {
    // INLINE = 2: len 2 stays inline, len 5 spills
    for len in [2usize, 5] {
        let mut v: SmallVec<String, 2, Global> = SmallVec::with_capacity_in(0, Global);
        for i in 0..len {
            v.push(i.to_string());
        }
        assert_eq!(v.spilled(), len > 2);

        let (length, raw, allocator) = v.into();
        assert_eq!(length.len(), len);
        assert_eq!(length.is_spilled(), len > 2);

        // SAFETY: parts come straight from `into_parts_with_allocator`.
        let rebuilt = unsafe { SmallVec::<String, 2, Global>::parts_in(length, raw, allocator) };
        assert_eq!(rebuilt.len(), len);
        assert_eq!(rebuilt[0], "0");
    }
}
