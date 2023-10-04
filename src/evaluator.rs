use std::sync::{Arc, Mutex};

pub use coco_rs::Problem;
use mahf::{
    population::AsSolutions, problems::Evaluate, state::State, Individual, SingleObjective,
};

use crate::{backends::Backend, instance::Instance, Context, Suite};

/// Evaluates [`Instance`]s.
///
/// Must be inserted manually during [`mahf::Configuration::optimize_with`] or
/// it will be inserted automatically during [`evaluate_suite`].
///
/// [`evaluate_suite`]: crate::evaluate_suite
pub struct StandardEvaluator<'s> {
    problem: Problem<'s>,
}

impl<'s> StandardEvaluator<'s> {
    /// Creates a new evaluator for the given suite and instance.
    pub fn new(suite: &'s mut Suite, instance: &Instance) -> Self {
        Self {
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

impl Evaluate for StandardEvaluator<'_> {
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

#[derive(Clone)]
pub struct ThreadSafeEvaluator<'s> {
    problem: Arc<Mutex<Problem<'s>>>,
}

#[allow(dead_code)]
impl<'s> ThreadSafeEvaluator<'s> {
    /// Creates a new evaluator for the given suite and instance.
    pub fn new(suite: &'s mut Suite, instance: &Instance) -> Self {
        Self {
            problem: Arc::new(Mutex::new(suite.problem_for_instance(instance))),
        }
    }

    pub fn evaluate_individuals(&mut self, individuals: &mut [Individual<Instance>]) {
        for individual in individuals {
            let mut out = [0.0];
            self.problem
                .lock()
                .expect("mutex could not be locked")
                .evaluate_function(individual.solution(), &mut out);
            individual.set_objective(SingleObjective::try_from(out[0]).unwrap());
        }
    }
}

impl Evaluate for ThreadSafeEvaluator<'_> {
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

pub struct AcceleratedEvaluator<'c, B: Backend> {
    problem: coco_accelerated::bbob::Problem,
    evaluator: coco_accelerated::bbob::Evaluator<'c, B>,
}

impl<'c, B: Backend> AcceleratedEvaluator<'c, B> {
    #[allow(dead_code)]
    pub fn new(context: &'c Context<B>, suite: &mut Suite, instance: &Instance) -> Self {
        let problem = suite.problem_for_instance(instance);
        let function = problem.function_index().0;
        let instance = problem.instance_index().0;
        let dimension = problem.dimension();

        let function = coco_accelerated::bbob::Function::from_repr(function).unwrap();

        let accelerated_problem =
            coco_accelerated::bbob::Problem::new(function, dimension, instance);
        let evaluator = accelerated_problem.evaluator(context);

        Self {
            problem: accelerated_problem,
            evaluator,
        }
    }

    pub fn evaluate_individuals(&mut self, individuals: &mut [Individual<Instance>]) {
        let solutions = individuals.as_solutions();
        let objective_values = self.evaluator.evaluate_iter(solutions);

        for (individual, objective_value) in individuals.iter_mut().zip(objective_values) {
            individual.set_objective(SingleObjective::try_from(objective_value).unwrap());
        }
    }
}

impl<'c, B: Backend> Evaluate for AcceleratedEvaluator<'c, B> {
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

impl<'c, B: Backend> Clone for AcceleratedEvaluator<'c, B> {
    fn clone(&self) -> Self {
        let problem = self.problem.clone();
        let evaluator = problem.evaluator(self.evaluator.context());

        Self { problem, evaluator }
    }
}
