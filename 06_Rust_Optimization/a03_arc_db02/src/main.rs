// https://stackoverflow.com/questions/44636833/is-it-possible-to-create-an-arct-from-a-vect
use std::sync::Arc;

fn main() {
    let v = vec![1, 2, 3];
    let y: Arc<&[u32]> = Arc::new(&v[..]);
    print!("{:?}", y)
}
