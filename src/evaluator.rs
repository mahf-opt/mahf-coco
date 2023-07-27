pub use coco_rs::Problem;
use mahf::{problems::Evaluate, state::State, Individual, SingleObjective};

use crate::{instance::Instance, Suite};

/// Evaluates [`Instance`]s.
///
/// Must be inserted manually during [`mahf::Configuration::optimize_with`] or
/// it will be inserted automatically during [`evaluate_suite`].
///
/// [`evaluate_suite`]: crate::evaluate_suite
pub struct InstanceEvaluator<'s> {
    problem: Problem<'s>,
}

impl<'s> InstanceEvaluator<'s> {
    /// Creates a new evaluator for the given suite and instance.
    pub fn new(suite: &'s mut Suite, instance: &Instance) -> Self {
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
