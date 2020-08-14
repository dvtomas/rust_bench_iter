This is what I get on my Core-i7

```
$ cargo bench
   Compiling bench_iter v0.1.0 (/home/td/projects-sandbox/rust_bench_iter)
    Finished bench [optimized] target(s) in 3.47s
     Running target/release/deps/bench_iter-e0df5c1fe8074663

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/release/deps/my_benchmark-7c1f3cac9736b209
iter                    time:   [123.41 ns 123.51 ns 123.62 ns]                 
                        change: [-28.213% -28.099% -27.980%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  1 (1.00%) high severe

for                     time:   [23.755 ns 23.777 ns 23.799 ns]                 
                        change: [+0.4685% +0.6315% +0.8007%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe
```
