use rayon::prelude::*;

fn main() {
    let mut arr = [0, 7, 9, 11];
    arr.par_iter_mut().for_each(|p| {
        let id = std::thread::current().id();
        println!("thread id : {id:?}");
        *p += 1;
    });
    println!("rayon let's go !!!");
}
