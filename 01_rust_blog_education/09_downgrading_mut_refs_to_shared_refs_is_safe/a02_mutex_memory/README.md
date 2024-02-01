# Result

```
cannot borrow `s` as immutable because it is also borrowed as mutable
  --> src/main.rs:25:5
   |

24 |     let str_ref = s.get_string(); // mut ref downgr...
   |                   - mutable borrow occurs here
25 |     s.mutate_string(); // str_ref invalidated, now ...
   |     ^ immutable borrow occurs here
26 |     dbg!(str_ref); // ❌ - as expected!
   |          ------- mutable borrow later used here


```

- Result

```bash
[src/main.rs:28] s = Struct {
    mutex: Mutex {
        data: "surprise!",
        poisoned: false,
        ..
    },
}

```


