/*
 * Copyright (c) 2023. Marvin Hansen <marvin.hansen@gmail.com> All rights reserved.
 */
use derive_more::{Constructor, Display};
use std::fmt::{Display, Formatter};
use std::time::Duration;

#[allow(dead_code)]
#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
pub struct Record{
    name: String,
    duration: Duration,
}

impl Record{

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn duration(&self) -> Duration {
        self.duration
    }
    pub fn new(name: String, duration: Duration) -> Self {
        Self { name, duration }
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Default, Display, Clone, Hash, Eq, PartialEq)]
pub enum TestField {
    #[default]
    TestPoint,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Constructor)]
pub struct TestPoints {
    first_test: TestField,
    second_test: TestField,
    third_test: TestField,
    fifth_test: TestField,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, Constructor)]
pub struct Node {
    id: u64,
    tempoid_vertex_id: u64,
    floor_pivots: TestPoints,
    camarilla_pivots: TestPoints,
    fibonacci_pivots: TestPoints,
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node: {:?}", self)
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, Display, Constructor)]
pub struct Edge {
    cost: usize,
}

impl Into<usize> for Edge {
    fn into(self) -> usize {
        self.cost
    }
}
