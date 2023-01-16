struct Test {
    a: i32,
}

impl Drop for Test {
    fn drop(&mut self) {
        println!("droped");
    }
}

static mut HELLO: Test = Test { a: 0 };

extern "C" fn __init_hello() {
    println!("init program");
}

extern "C" fn __fini_hello() {
    unsafe {
        drop(&HELLO);
    }

    println!("finish program")
}

#[link_section = "__DATA,__mod_init_func"]
#[used]
pub static __INIT_SECTION: extern "C" fn() = __init_hello;

#[link_section = "__DATA,__mod_init_func"]
#[used]
pub static __FINI_SECTION: extern "C" fn() = __fini_hello;

fn main() {
    unsafe {
        println!("HELLO a: {}", HELLO.a);
    }
    println!("main Hello, world! C ~~~ Best best");
}
