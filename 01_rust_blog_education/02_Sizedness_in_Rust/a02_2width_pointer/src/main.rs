use std::mem::size_of;

const WIDTH: usize = size_of::<&()>();
const DOUBLE_WIDTH: usize = 2 * WIDTH;

fn main() {
    // data length stored in type
    // an[i32; 3] is an array of three i32s
    let nums: &[i32; 3] = &[1, 2, 3];

    // single-width pointer
    assert_eq!(WIDTH, size_of::<&[i32; 3]>());

    let mut sum = 0;

    // can iterate over nums safely
    // Rust knows it's examtly 3 elements
    for num in nums {
        sum += num;
    }

    assert_eq!(6, sum);

    // unsized coercion from [i32; 3] to [i32]
    // data length now stored in pointer
    let nums: &[i32] = &[1, 2, 3];

    // double-width pointer required to also store data length
    assert_eq!(DOUBLE_WIDTH, size_of::<&[i32]>());

    let mut sum = 0;

    // can iterate over nums safely
    // Rust knows it's exactly 3 elements
    for num in nums {
        sum += num;
    }

    assert_eq!(6, sum);
}
