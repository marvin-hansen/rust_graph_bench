/*
 * Copyright (c) 2023. Marvin Hansen <marvin.hansen@gmail.com> All rights reserved.
 */
use hypergraph::Hypergraph;
use petgraph::csr::Csr;
use petgraph::matrix_graph::MatrixGraph;
use std::thread;
use std::time::{Duration, Instant};
mod structs;
use structs::*;

fn main() {
    // number of nodes added to the graph
    let k = 1_000_000;

    let csr_time = time_csr_graph(k);
    let mtx_time = time_matrix_graph(k);
    let hyper_time = time_hyper_graph(k);

    println!("Number of nodes added: {}", k);
    println!();
    print_duration("CsrGraph", &csr_time);
    print_duration("MatrixGraph", &mtx_time);
    print_duration("HyperGraph", &hyper_time);

    let diff_hyper_mtx = hyper_time - csr_time;
    println!("Diff: HyperGraph - CsrGraph : {:?} ", diff_hyper_mtx);
    println!();

    let diff_hyper_mtx = hyper_time - mtx_time;
    println!("Diff: HyperGraph - MatrixGraph : {:?} ", diff_hyper_mtx);
    println!();

    // Wait one second to read memory usage
    thread::sleep(Duration::from_secs(1));
}

fn time_matrix_graph(k: u64) -> Duration {
    let start = Instant::now();
    let mut g: MatrixGraph<Node, Edge> = petgraph::matrix_graph::MatrixGraph::new();

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

    return start.elapsed();
}

fn time_csr_graph(k: u64) -> Duration {
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

    return start.elapsed();
}

pub fn print_duration(msg: &str, elapsed: &Duration) {
    println!("{} took: {:?} ", msg, elapsed);
    println!();
}

fn time_hyper_graph(k: u64) -> Duration {
    let start = Instant::now();
    let mut g = Hypergraph::<Node, Edge>::new();

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

    return start.elapsed();
}
