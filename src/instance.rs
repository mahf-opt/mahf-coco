use coco_rs::Problem;
use mahf::{
    framework::SingleObjective,
    problems::{self},
    state::common::EvaluatorInstance,
};
use std::ops::RangeInclusive;

#[derive(serde::Serialize)]
pub struct Instance {
    pub(crate) function_idx: usize,
    pub(crate) instance_idx: usize,
    pub(crate) dimension_idx: usize,

    name: String,
    dimension: usize,
    ranges_of_interest: Vec<RangeInclusive<f64>>,
    final_target_value: f64,
}

impl Instance {
    pub fn format_name(&self) -> String {
        self.name.clone()
    }

    pub fn from(problem: &Problem) -> Self {
        let name = problem.id().to_string();
        let dimension = problem.dimension();
        let ranges_of_interest = problem.get_ranges_of_interest();
        let final_target_value = problem.final_target_value();

        Instance {
            function_idx: problem.function_index(),
            instance_idx: problem.instance_index(),
            dimension_idx: problem.dimension_index(),

            name,
            dimension,
            ranges_of_interest,
            final_target_value,
        }
    }
}

impl problems::Problem for Instance {
    type Encoding = Vec<f64>;
    type Objective = SingleObjective;

    fn name(&self) -> &str {
        "Coco"
    }

    fn default_evaluator<'a>(&self) -> EvaluatorInstance<'a, Self> {
        unimplemented!("the evaluator has to be inserted manually")
    }
}

impl problems::VectorProblem for Instance {
    type T = f64;

    fn dimension(&self) -> usize {
        self.dimension
    }
}

impl problems::LimitedVectorProblem for Instance {
    fn range(&self, dimension: usize) -> std::ops::Range<Self::T> {
        let range = self.ranges_of_interest[dimension].clone();

        let (start, end) = range.into_inner();
        start..end
    }
}

impl problems::HasKnownTarget for Instance {
    fn target_hit(&self, target: SingleObjective) -> bool {
        target.value() <= self.final_target_value
    }
}
