struct Cons<First, Second>(First, Second);

trait HCons: Sized {
    fn cons<T>(self, other: T) -> Cons<Self, T> {
        Cons(self, other)
    }
}

impl<T: Sized> HCons for T {}

// This is a hack to get around the fact that manually implementing the `Fn`
// traits is currently unstable.
trait Callable {
    fn call(self);
}

impl<F: Fn() -> ()> Callable for F {
    fn call(self) {
        self()
    }
}

impl<First: Callable, Second: Callable> Callable for Cons<First, Second> {
    fn call(self) {
        self.0.call();
        self.1.call();
    }
}

fn call_all_fns_no_alloc<T: Callable>(fns: T) {
    fns.call();
}

fn main() {
    let first_fn = || {
        println!("Hello!");
    };
    let second_fn = || {
        println!("World!");
    };

    call_all_fns_no_alloc(first_fn.cons(second_fn));
}
