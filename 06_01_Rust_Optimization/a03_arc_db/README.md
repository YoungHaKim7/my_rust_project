# Result

```
db Person { name: "Gyoung", age: 40 }
db Person { name: "John Doe", age: 30 }

$ cargo size

    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
warning: fields `name` and `age` are never read
__TEXT	__DATA	__OBJC	others	dec	hex
294912	16384	0	4295213056	4295524352	100088000	

```
