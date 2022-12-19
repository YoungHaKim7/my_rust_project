#[no_mangle]
pub extern "C" fn addition(a: u32, b: u32) -> u32 {
    a + b
}
fn main() {
    unsafe {
    println!("1 + 3 =  {} ", addition(1, 3));
    }
}
