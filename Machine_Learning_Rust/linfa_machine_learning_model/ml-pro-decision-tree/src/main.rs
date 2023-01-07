use linfa::prelude::*;
use linfa_trees::DecisionTree;

fn main() {
    let (train, test) = linfa_datasets::iris().split_with_ratio(0.9);

    let model = DecisionTree::params().fit(&train).unwrap();

    let predictions = model.predict(&test);

    println!("{:?}", predictions);
    println!("{:?}", test.targets);
}
