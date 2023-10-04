use std::ops::RangeInclusive;

use coco_rs::suite::{Name, ProblemIdx};
use itertools::Itertools;

use crate::Instance;

/// A COCO benchmark suite.
///
/// It's a thin wrapper around [`coco_rs::Suite`] to simplify integration with MAHF.
#[derive(Clone)]
pub struct Suite {
    pub(crate) inner: coco_rs::Suite,
}

pub enum Instances {
    Year(usize),
    Explicit(Vec<usize>),
    Range(RangeInclusive<usize>),
}

impl ToString for Instances {
    fn to_string(&self) -> String {
        match self {
            Instances::Year(year) => format!("year:{year}"),
            Instances::Explicit(list) => list.iter().map(ToString::to_string).join(","),
            Instances::Range(range) => format!("{}-{}", range.start(), range.end()),
        }
    }
}

#[derive(Default, Clone)]
pub struct Options {
    dimensions: Option<Vec<usize>>,
    dimension_indices: Option<Vec<usize>>,
    function_indices: Option<Vec<usize>>,
    instance_indices: Option<Vec<usize>>,
}


impl Options {
    pub fn new() -> Self {
        Self {
            dimensions: None,
            dimension_indices: None,
            function_indices: None,
            instance_indices: None,
        }
    }

    pub fn with_dimensions(mut self, dimensions: impl IntoIterator<Item = usize>) -> Self {
        self.dimensions = Some(dimensions.into_iter().collect());
        self
    }

    pub fn with_dimension_indices(
        mut self,
        dimension_indices: impl IntoIterator<Item = usize>,
    ) -> Self {
        self.dimension_indices = Some(dimension_indices.into_iter().collect());
        self
    }

    pub fn with_function_indices(
        mut self,
        function_indices: impl IntoIterator<Item = usize>,
    ) -> Self {
        self.function_indices = Some(function_indices.into_iter().collect());
        self
    }

    pub fn with_instance_indices(
        mut self,
        instance_indices: impl IntoIterator<Item = usize>,
    ) -> Self {
        self.instance_indices = Some(instance_indices.into_iter().collect());
        self
    }
}

impl ToString for Options {
    fn to_string(&self) -> String {
        let mut options = Vec::new();

        if let Some(dimensions) = &self.dimensions {
            let option = dimensions.iter().map(ToString::to_string).join(",");
            options.push(format!("dimensions:{option}"));
        }

        if let Some(dimension_indices) = &self.dimension_indices {
            let option = dimension_indices.iter().map(ToString::to_string).join(",");
            options.push(format!("dimension_indices:{option}"));
        }

        if let Some(function_indices) = &self.function_indices {
            let option = function_indices.iter().map(ToString::to_string).join(",");
            options.push(format!("function_indices:{option}"));
        }

        if let Some(instance_indices) = &self.instance_indices {
            let option = instance_indices.iter().map(ToString::to_string).join(",");
            options.push(format!("instance_indices:{option}"));
        }

        options.join(" ")
    }
}

impl Suite {
    /// Creates a new suite with the given name.
    pub fn new(name: Name) -> Self {
        Suite {
            inner: coco_rs::Suite::new(name, "", "").unwrap(),
        }
    }

    /// Create a new suite with the given name, instances and options.
    ///
    /// Fails if the provided instances don't exist.
    pub fn with_options(
        name: Name,
        instances: Option<&Instances>,
        options: Option<&Options>,
    ) -> Option<Self> {
        let instances = instances.map(ToString::to_string).unwrap_or("".to_string());
        let options = options.map(ToString::to_string).unwrap_or("".to_string());
        coco_rs::Suite::new(name, &instances, &options).map(|inner| Self { inner })
    }

    /// Returns the total number of problems in the suite.
    pub fn number_of_problems(&self) -> usize {
        self.inner.number_of_problems()
    }

    /// Returns the [`coco_rs::Problem`] for a specific instance.
    pub(crate) fn problem_for_instance(&mut self, instance: &Instance) -> coco_rs::Problem {
        self.inner
            .problem(ProblemIdx(instance.problem_idx))
            .unwrap()
    }
}

impl Iterator for Suite {
    type Item = Instance;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Instance::from(&self.inner.next_problem(None)?))
    }
}
