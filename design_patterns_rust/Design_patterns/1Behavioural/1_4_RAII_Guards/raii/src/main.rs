use std::ops::Deref;

struct Foo {}

struct Mutex<T> {

}
struct MutexGuard<'a, T: 'a> {
    data: &'a T,
}

impl<T> Mutex<T> {
    fn lock(&self) -> MutexGuard<T> {

    MutexGuard {
            data: self,
        }
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {

    }
}

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.data
    }
}

fn baz(x: Mutex<Foo>) {
    let xx = x.lock();
    xx.foo();
}

fn main() {

    println!("Hello, world!");
}
