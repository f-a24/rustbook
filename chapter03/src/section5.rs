/*
Section3-5: テスト
*/

/// This function adds 2 numbers.
/// 
/// # Example
/// 
/// ```
/// use chapter03::section5::add;
/// 
/// add(1, 2);
/// ```
#[allow(dead_code)]
pub fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

#[test]
fn test_add() {
    assert_eq!(0, add(0, 0));
    assert_eq!(1, add(0, 1));
    assert_eq!(1, add(1, 0));
    assert_eq!(2, add(1, 1));
}

#[test]
fn assert_sample() {
    assert!(true);
    assert!(false, "panic! value={}", false);
    assert_eq!(true, true);
    assert_ne!(true, false);
    assert_eq!(true, false, "panic! valut=({}, {})", true, false);
}

#[test]
#[should_panic]
fn test_panic() {
    panic!("expected panic");
}

#[test]
#[ignore]
fn test_add_ignored() {
    assert_eq!(-2, add(-1, -1));
}