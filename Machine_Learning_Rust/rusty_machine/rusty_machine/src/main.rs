use rusty_machine::linalg::Matrix;
use rusty_machine::linalg::Vector;
use rusty_machine::learning::gp::GaussianProcess;
use rusty_machine::learning::gp::ConstMean;
use rusty_machine::learning::toolkit::kernel;
use rusty_machine::learning::SupModel;

fn main () {
    // First we'll get some data.

    // Some example training data.
    let inputs = Matrix::new(3,3,vec![1.,1.,1.,2.,2.,2.,3.,3.,3.]);
    let targets = Vector::new(vec![0.,1.,0.]);

    // Some example test data.
    let test_inputs = Matrix::new(2,3, vec![1.5,1.5,1.5,2.5,2.5,2.5]);

    // Now we'll set up our model.
    // This is close to the most complicated a model in rusty-machine gets!

    // A squared exponential kernel with lengthscale 2, and amplitude 1.
    let ker = kernel::SquaredExp::new(2., 1.);

    // The zero function
    let zero_mean = ConstMean::default();

    // Construct a GP with the specified kernel, mean, and a noise of 0.5.
    let mut gp = GaussianProcess::new(ker, zero_mean, 0.5);


    // Now we can train and predict from the model.

    // Train the model!
    gp.train(&inputs, &targets).unwrap();

    // Predict output from test datae]
    let outputs = gp.predict(&test_inputs).unwrap();

}
