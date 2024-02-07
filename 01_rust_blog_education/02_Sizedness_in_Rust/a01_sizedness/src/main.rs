use std::mem::size_of;

fn main() {
    // primitives
    assert_eq!(4, size_of::<i32>()); // 4bytes
    assert_eq!(8, size_of::<i64>()); // 8bytes

    // tuple
    assert_eq!(8, size_of::<(i32, i32)>());

    // arrays
    assert_eq!(0, size_of::<[i32; 0]>());
    assert_eq!(12, size_of::<[i32; 3]>());

    struct Point {
        x: i32,
        y: i32,
    }

    //struct
    assert_eq!(8, size_of::<Point>());

    // enum
    assert_eq!(8, size_of::<Option<i32>>());

    // get point width, will be
    // 4 bytes wide on 32-bit target or
    // 8 bytes wide on 64-bit target
    const WIDTH: usize = size_of::<&()>();

    // pointers to sized types are 1 width
    assert_eq!(WIDTH, size_of::<&i32>());
    assert_eq!(WIDTH, size_of::<&mut i32>());
    assert_eq!(WIDTH, size_of::<Box<i32>>());
    assert_eq!(WIDTH, size_of::<fn(i32) -> i32>());

    const DOUBLE_WIDTH: usize = 2 * WIDTH;

    // unsized struct
    struct Unsized {
        unsized_field: [i32],
    }

    // pointers to unsized types are 2 widths
    assert_eq!(DOUBLE_WIDTH, size_of::<&str>()); // slice
    assert_eq!(DOUBLE_WIDTH, size_of::<&[i32]>()); // slice
    assert_eq!(DOUBLE_WIDTH, size_of::<&dyn ToString>()); // trait object
    assert_eq!(DOUBLE_WIDTH, size_of::<Box<&dyn ToString>>()); // trait object
    assert_eq!(DOUBLE_WIDTH, size_of::<&Unsized>()); // user-defined unsized type
                                                     //
                                                     // unsized type
                                                     // size_of::<str>(); // compile error
                                                     // size_of::<[i32]>(); // compile error
                                                     // size_of::<dyn ToString>(); // compile error
                                                     // size_of::<Unsized>(); // compile error
}
