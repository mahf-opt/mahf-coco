use crate::{instance::Instance, Suite};
pub use coco_rs::Problem;
use mahf::{
    framework::{Individual, SingleObjective},
    problems::Evaluator,
    state::{common::EvaluatorInstance, State},
};

pub struct InstanceEvaluator<'s> {
    problem: Problem<'s>,
}

impl<'s> InstanceEvaluator<'s> {
    pub fn new(suite: &'s mut Suite, instance: &Instance) -> EvaluatorInstance<'s, Instance> {
        EvaluatorInstance::new(InstanceEvaluator {
            problem: suite.problem_for_instance(instance),
        })
    }
}

impl Evaluator for InstanceEvaluator<'_> {
    type Problem = Instance;

    fn evaluate(
        &mut self,
        _problem: &Self::Problem,
        _state: &mut State<Self::Problem>,
        individuals: &mut [Individual<Self::Problem>],
    ) {
        for individual in individuals {
            let mut out = [0.0];
            self.problem
                .evaluate_function(individual.solution(), &mut out);
            individual.evaluate(SingleObjective::try_from(out[0]).unwrap())
        }
    }
}