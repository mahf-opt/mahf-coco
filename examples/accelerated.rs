use std::slice::from_mut;

use mahf::{problems::VectorProblem, Individual, Problem};
use mahf_coco::{backends::C, AcceleratedEvaluator, Context, Name, Options, Suite};
use once_cell::sync::Lazy;

static CONTEXT: Lazy<Context<C>> = Lazy::new(Context::default);

fn main() {
    coco_rs::LogLevel::Error.set();

    let options = Options::new()
        .with_dimensions([2, 5, 10])
        .with_function_indices([1, 5, 7])
        .with_instance_indices(1..10);
    let mut suite = Suite::with_options(Name::Bbob, None, Some(&options)).unwrap();

    while let Some(instance) = suite.next() {
        let mut evaluator = AcceleratedEvaluator::new(&CONTEXT, &mut suite, &instance);

        let mut zero = Individual::new_unevaluated(vec![0.0; instance.dimension()]);
        evaluator.evaluate_individuals(from_mut(&mut zero));
        println!(
            "{}(0.0, ..., 0.0) = {}",
            instance.name(),
            zero.objective().value()
        );
    }
}
