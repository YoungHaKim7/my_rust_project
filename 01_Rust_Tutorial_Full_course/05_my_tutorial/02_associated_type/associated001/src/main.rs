trait Contains {
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
}

fn difference<A, B, C>(container: &C) -> i32
where
    C: Contains<A, B>,
{
    todo!();
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!(
        "Does container contain {} and {} : {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );

    println!("Fistst number: {}", container.first());
    println!("Lastnumber: {}", container.last());

    println!("THe difference is: {}", difference(&container));
}
