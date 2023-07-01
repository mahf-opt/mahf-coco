use crate::{instance::Instance, Suite};
pub use coco_rs::Problem;
use mahf::{
    framework::{Individual, SingleObjective},
    problems::Evaluate,
    state::{common::EvaluatorInstance, State},
};

/// Evaluates [Instance]s.
///
/// Must be inserted manually during [mahf::Configuration::optimize_with] or
/// it will be inserted automatically during [crate::evaluate_suite].
pub struct InstanceEvaluator<'s> {
    problem: Problem<'s>,
}

impl<'s> InstanceEvaluator<'s> {
    /// Creates a new evaluator for the given suite and instance.
    pub fn new(suite: &'s mut Suite, instance: &Instance) -> EvaluatorInstance<'s, Instance> {
        EvaluatorInstance::new(InstanceEvaluator {
            problem: suite.problem_for_instance(instance),
        })
    }

    /// Creates a new instance for the given suite and instance.
    ///
    /// This is only needed for custom evaluation procedures.
    /// Normally you want to add it directly to the [State] as
    /// an [EvaluatorInstance], so using [InstanceEvaluator::new] is preferred.
    pub fn new_raw(suite: &'s mut Suite, instance: &Instance) -> Self {
        InstanceEvaluator {
            problem: suite.problem_for_instance(instance),
        }
    }

    pub fn evaluate_individuals(&mut self, individuals: &mut [Individual<Instance>]) {
        for individual in individuals {
            let mut out = [0.0];
            self.problem
                .evaluate_function(individual.solution(), &mut out);
            individual.set_objective(SingleObjective::try_from(out[0]).unwrap());
        }
    }
}

impl Evaluate for InstanceEvaluator<'_> {
    type Problem = Instance;

    fn evaluate(
        &mut self,
        _problem: &Self::Problem,
        _state: &mut State<Self::Problem>,
        individuals: &mut [Individual<Self::Problem>],
    ) {
        self.evaluate_individuals(individuals);
    }
}
