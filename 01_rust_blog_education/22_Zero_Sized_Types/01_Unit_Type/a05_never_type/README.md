# Result

- https://stackoverflow.com/questions/21747136/how-do-i-print-in-rust-the-type-of-a-variable

```
[src/main.rs:33:5] example3(true) = "str"
i32
[src/main.rs:35:5] example4() = 1243
```

```
[src/main.rs:47:5] example3(true) = "str"
i32
[src/main.rs:49:5] example4() = 1243
[src/main.rs:50:5] example5(&[10]) = [
    10,
]
[src/main.rs:51:5] example5(&[11]) = []
[src/main.rs:52:5] example5(&[8]) = [
    8,
]
[src/main.rs:53:5] example5(&[-8]) = []  
```
