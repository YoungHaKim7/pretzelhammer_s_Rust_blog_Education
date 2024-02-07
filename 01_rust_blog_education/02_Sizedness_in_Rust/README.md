# Sizedness in Rust

- https://github.com/pretzelhammer/rust-blog/blob/master/posts/sizedness-in-rust.md

# 정렬 시켜시켜서   

- u8 1byte
- u16 2byte
- u32 4byte


- ctx

- context


- 크기를 알면  Sized types can be passed around by value or by reference

- 크기 모르면 Re  & & 이거안된다.




```rs
-> thin pointer

fn (xjkxjk: djsklsj) -> i32 {}

=> fat pointer

match =>
```

- trait object pointers are double-width 

- because they store
1.    a pointer to the data 

2. and a pointer to a vtable


- slices are double-width  []  []   index[0]  0 1 2 3 
  - because they store 
1. a pointer to the array 
2. and the number of elements in the array



- Sized types can be passed around by value or by reference.

- Since unsized types can't be placed on the stack they can only be passed around by reference. 



```rs
// 1 width 	
//single unit of measurement of pointer width

// pointers to sized types are 1 width~~~~~~~~~~~~
    assert_eq!(WIDTH, size_of::<&i32>());
    assert_eq!(WIDTH, size_of::<&mut i32>());
    assert_eq!(WIDTH, size_of::<Box<i32>>());  // easy rust 매직 보자기?? 
    assert_eq!(WIDTH, size_of::<fn(i32) -> i32>());


// ~~~~~~~~~~~2 width ~~~~~~~~~~~~
    // pointers to unsized types are 2 widths
assert_eq!(DOUBLE_WIDTH, size_of::<&str>()); // slice
assert_eq!(DOUBLE_WIDTH, size_of::<&[i32]>()); // slice
assert_eq!(DOUBLE_WIDTH, size_of::<&dyn ToString>()); // trait object
assert_eq!(DOUBLE_WIDTH, size_of::<Box<dyn ToString>>()); // trait object
assert_eq!(DOUBLE_WIDTH, size_of::<&Unsized>()); // user-defined unsized type
```
