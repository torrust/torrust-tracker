//! Example File
//!
//!

#[allow(dead_code)]
enum EnumExample {
    Time,
    Look,
    Life(usize),
    Go,
}

#[test]
fn example() {
    let a_enum = EnumExample::Time;
    let _ = EnumExample::Look;

    let it_is_time = match a_enum {
        EnumExample::Time => true,
        EnumExample::Look | EnumExample::Life(_) | EnumExample::Go => false,
    };

    assert!(it_is_time);

    // panic!();
}
