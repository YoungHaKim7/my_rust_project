fn main() {
    let a: f32 = 0.1;
    let mut b: f32 = a * 10_f32;
    let c: f32 = 1_0_f32;

    println!("a: {a}");
    println!("b = a* 10 : {b}");
    println!("c = 1.0 : {c}");

    println!("b: {:b}", b.to_bits());
    println!("c: {:b}", c.to_bits());

    let mut d: f32 = 0_f32;
    for i in 0..10 {
        d = d + 0.1_f32;
    }

    println!("d = 0.1 + 0.1 + ..... + 0.1 (10)");
    println!("d: {:b}", d.to_bits());
    println!("d: {d}");
}

// Result:
// a: 0.1
// b = a* 10 : 1
// c = 1.0 : 10
// b: 111111100000000000000000000000
// c: 1000001001000000000000000000000
// d = 0.1 + 0.1 + ..... + 0.1 (10)
// d: 111111100000000000000000000001
// d: 1.0000001
