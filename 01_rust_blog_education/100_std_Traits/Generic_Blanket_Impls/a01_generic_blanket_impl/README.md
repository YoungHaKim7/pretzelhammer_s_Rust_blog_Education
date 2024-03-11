# Result

```
cargo t -- --nocapture
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/a01_generic_blanket_impl-6855c3ec63997b05)

running 1 test
my_val even? = true
test tests::is_even_impl ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests a01_generic_blanket_impl

running 0 tests

```

```bash

cargo nextest run --nocapture
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
    Starting 1 test across 1 binary
       START             a01_generic_blanket_impl tests::is_even_impl

running 1 test
my_val even? = true
test tests::is_even_impl ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

        PASS [   0.001s] a01_generic_blanket_impl tests::is_even_impl
------------
     Summary [   0.002s] 1 test run: 1 passed, 0 skipped
```
