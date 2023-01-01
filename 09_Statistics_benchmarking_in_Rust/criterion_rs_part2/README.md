# Result:

```
$ cargo criterion

    Finished bench [optimized] target(s) in 11.91s


fib 20                  time:   [19.676 µs 19.683 µs 19.693 µs]

fibonacci/Recursive     time:   [311.01 ps 311.18 ps 311.37 ps]
fibonacci/Iterative     time:   [6.8313 ns 6.8332 ns 6.8354 ns]

Fibonacci3/Recursive/20 time:   [312.40 ps 312.63 ps 312.88 ps]
Fibonacci3/Iterative/20 time:   [6.8426 ns 6.8462 ns 6.8508 ns]
Fibonacci3/Recursive/21 time:   [310.66 ps 310.77 ps 310.89 ps]
Fibonacci3/Iterative/21 time:   [7.1415 ns 7.1437 ns 7.1463 ns]

from_elem/1024          time:   [173.33 ns 173.74 ns 174.15 ns]
                        thrpt:  [5.4763 GiB/s 5.4891 GiB/s 5.5019 GiB/s]
from_elem/2048          time:   [349.00 ns 349.15 ns 349.34 ns]
                        thrpt:  [5.4599 GiB/s 5.4628 GiB/s 5.4651 GiB/s]
from_elem/4096          time:   [688.29 ns 688.56 ns 688.86 ns]
                        thrpt:  [5.5377 GiB/s 5.5401 GiB/s 5.5423 GiB/s]
from_elem/8192          time:   [1.3705 µs 1.3714 µs 1.3723 µs]
                        thrpt:  [5.5594 GiB/s 5.5632 GiB/s 5.5668 GiB/s]
from_elem/16384         time:   [2.7412 µs 2.7499 µs 2.7587 µs]
                        thrpt:  [5.5312 GiB/s 5.5489 GiB/s 5.5664 GiB/s]


```

https://en.wikipedia.org/wiki/Metric_prefix

