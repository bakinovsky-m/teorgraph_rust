extern crate graph_tools;

fn main() {
    let mut g = graph_tools::graph::Graph::new();

    let ac = 5;
    let v1 = vec![0, 0, 3, 4];
    let v2 = vec![1, 2, 2, 2];

    g.fill(ac, v1, v2);

    let dist = g.bfs(0);

    for i in 0..dist.len() {
    	print!("{}, ", dist[i]);
    }
    println!("");
}

trait Bfsable{
	fn bfs(& mut self, start: i32) -> Vec<i32>;
	fn get_neighbours(&self, node: i32) -> Vec<i32>;
}

impl Bfsable for graph_tools::graph::Graph{
	fn bfs(& mut self, start: i32) -> Vec<i32>{
		let mut q = graph_tools::queue::Queue::new();
		let mut mark: Vec<i32> = Vec::new();
		let mut dist: Vec<i32> = Vec::new();
		mark.resize(self.IJ.len(), 0);
		dist.resize(self.IJ.len(), 0);

		q.push(start);
		mark[start as usize] = 1;

		while q.len() != 0 {
			let v = match q.pop() {
				None => break,
				Some(n) => n,
			};

			let neighbours = self.get_neighbours(v);

			for i in 0..neighbours.len(){
				if mark[neighbours[i] as usize] == 0 {
					mark[neighbours[i] as usize] = 1;
					dist[neighbours[i] as usize] = dist[v as usize] + 1;
					q.push(neighbours[i]);
				}
			}
		}

		for i in 0..dist.len() {
			if dist[i] == 0 && i != (start as usize) {
				dist[i] = std::usize::MAX as i32;
			}
		}

		return dist;
	}

	fn get_neighbours(&self, node: i32) -> Vec<i32>{
		let mut res = Vec::new();
		let mut v = self.H[node as usize];

		while v != -1{
			res.push(self.IJ[self.IJ.len() - 1 - (v as usize)]);
			v = self.L[v as usize];
		}

		return res;
	}
}