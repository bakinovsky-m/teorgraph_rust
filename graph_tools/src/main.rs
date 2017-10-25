extern crate graph_tools;

#[allow(unused_variables)]
fn main() {
	let mut g = graph_tools::graph::Graph::new();
	let q = graph_tools::queue::Queue::new();
	let arcs_count: i32 = 5;
	let v1: Vec<i32> = vec![0, 0, 3, 4];
	let v2: Vec<i32> = vec![1, 2, 2, 2];

	g.fill(arcs_count, v1, v2);

	println!("{}", g);
}