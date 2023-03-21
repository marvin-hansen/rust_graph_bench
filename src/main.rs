/*
 * Copyright (c) 2023. Marvin Hansen <marvin.hansen@gmail.com> All rights reserved.
 */
use hypergraph::Hypergraph;
use petgraph::csr::Csr;
use petgraph::matrix_graph::MatrixGraph;
use std::time::{Instant};
mod structs;
use structs::*;

fn main()
{
    // number of nodes added to the graph
    let k = 1_000_000;
    let with_capacity = false;
    let mut report: Vec<Record> = Vec::new();

    // CSR doesn't have built-in pre-allocation
    let csr_time = time_csr_graph(k);
    report.push(csr_time);

    // Matrixgraph slows down considerably with pre-allocation
    let mtx_time = time_matrix_graph(k, with_capacity);
    report.push(mtx_time);

    // Hypergraph inserts faster with pre-allocation
    let hyper_time = time_hyper_graph(k, with_capacity);
    report.push(hyper_time);

    println!("Number of nodes added: {}", k);
    println!();
    print_duration(report);

}

fn time_matrix_graph(
    k: u64,
    with_capacity: bool
)
    -> Record
{
    let start = Instant::now();
    let mut g: MatrixGraph<Node, Edge> = if with_capacity {
        let node_capacity = (k*2) as usize;
        MatrixGraph::with_capacity(node_capacity)
    } else {
        MatrixGraph::new()
    };

    for i in 1..k {
        let node = Node::new(
            i,
            2,
            TestPoints::default(),
            TestPoints::default(),
            TestPoints::default(),
        );
        g.add_node(node);
    }

    return Record::new(String::from("Matrix Graph"), start.elapsed());
}

fn time_csr_graph(
    k: u64
)
    -> Record
{
    let start = Instant::now();

    let mut g: Csr<Node, Edge> = Csr::default();

    for i in 1..k {
        let node = Node::new(
            i,
            2,
            TestPoints::default(),
            TestPoints::default(),
            TestPoints::default(),
        );
        g.add_node(node);
    }

    return Record::new(String::from("CSR Graph"), start.elapsed());
}

fn time_hyper_graph(
    k: u64,
    with_capacity: bool
)
    -> Record
{
    let start = Instant::now();

    let mut g: Hypergraph<Node, Edge> = if with_capacity {
        let capacity = (k*2) as usize;
        Hypergraph::<Node, Edge>::with_capacity(capacity, capacity)
    } else {
        Hypergraph::<Node, Edge>::new()
    };

    for i in 1..k {
        let node = Node::new(
            i,
            2,
            TestPoints::default(),
            TestPoints::default(),
            TestPoints::default(),
        );
        let _ = g.add_vertex(node);
    }

    return Record::new(String::from("Hyper Graph"), start.elapsed());
}


pub fn print_duration(
    mut report: Vec<Record>
)
{
    report.sort_by(|a, b| a.duration().cmp(&b.duration()));

    for r in report.iter_mut() {
        println!("{} took: {:?}", r.name(), r.duration());
    }
    println!();
}