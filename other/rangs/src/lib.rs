use std::ops::Range;

/// 如果两个范围重合，就返回true
///
///     assert_eq!(rangs::overlap(0..7, 3..10), true);
///     assert_eq!(rangs::overlap(1..5, 101..105), false);
///
/// 如果有一个范围为空，则不认为是重合
///
///     assert_eq!(rangs::overlap(0..0, 0..10), false);
///
pub fn overlap(r1: Range<usize>, r2: Range<usize>) -> bool {
    r1.start < r1.end && r2.start < r2.end && r1.start < r2.end && r2.start < r1.end
}
