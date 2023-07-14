use coco_rs::Problem;
use mahf::{
    problems::{self},
    SingleObjective,
};
use std::ops::Range;

/// A COCO problem instance.
///
/// This represents a [coco_rs::Problem].
/// Evaluating this requires a [crate::InstanceEvaluator] in the [mahf::State].
/// It can be inserted when calling [mahf::Configuration::optimize_with].
/// You can take a look at [crate::evaluate_suite] for an example.
#[derive(serde::Serialize)]
pub struct Instance {
    pub(crate) function_idx: usize,
    pub(crate) instance_idx: usize,
    pub(crate) dimension_idx: usize,

    name: String,
    dimension: usize,
    ranges_of_interest: Vec<Range<f64>>,
    final_target_value: f64,
}

impl Instance {
    /// Creates an [Instance] from a [Problem].
    pub(crate) fn from(problem: &Problem) -> Self {
        let name = problem.id().to_string();
        let dimension = problem.dimension();
        let ranges_of_interest = problem.get_ranges_of_interest();
        let final_target_value = problem.final_target_value();

        // TODO: update this once MAHF can distinguish inclusive and exclusive domains.
        let ranges_of_interest = ranges_of_interest
            .into_iter()
            .map(|range| (*range.start())..(*range.end()))
            .collect();

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
        &self.name
    }
}

impl problems::VectorProblem for Instance {
    type Element = f64;

    fn dimension(&self) -> usize {
        self.dimension
    }
}

impl problems::LimitedVectorProblem for Instance {
    fn domain(&self) -> Vec<std::ops::Range<Self::Element>> {
        self.ranges_of_interest.clone()
    }
}

impl problems::KnownOptimumProblem for Instance {
    fn known_optimum(&self) -> SingleObjective {
        self.final_target_value.try_into().unwrap()
    }
}
