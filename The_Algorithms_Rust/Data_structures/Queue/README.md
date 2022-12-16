# result : 

- cargo test or cargo nextest run

```
$ cargo nextest run

    Finished test [unoptimized + debuginfo] target(s) in 0.00s
    Starting 4 tests across 1 binaries
        PASS [   0.003s] queue tests::test_dequeue
        PASS [   0.003s] queue tests::test_peek_front
        PASS [   0.004s] queue tests::test_size
        PASS [   0.004s] queue tests::test_enqueue
------------
     Summary [   0.004s] 4 tests run: 4 passed, 0 skipped

```
