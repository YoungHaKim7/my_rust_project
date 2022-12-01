fn bar() -> Result<(), ()> {
    struct Foo;

    impl Drop for Foo {
        fn drop(&mut self) {
            println!("exit");
        }
    }

    let _exit = Foo;

    Ok(())
}
fn main() {
    bar();
    println!("Hello, world!");
}
