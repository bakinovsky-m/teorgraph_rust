extern crate graph_tools;

fn main() {
    let mut g = graph_tools::graph::Graph::new();

    let ac = 5;
    let v1 = vec![0, 0, 3, 4];
    let v2 = vec![1, 2, 2, 2];

    g.fill(ac, v1, v2);

    for i in 0..g.arcs_count{
    	g.dfs(i, i);
    }
    println!("{}", g);
}

trait Dfsable{
	fn dfs(& mut self, node: i32, color: i32);
}

impl Dfsable for graph_tools::graph::Graph{
	fn dfs(& mut self, node: i32, color: i32){
		if self.compSzyaznDFS[node as usize] == -1{
			self.compSzyaznDFS[node as usize] = color;
			for i in 0..(self.IJ.len() / 2){
				if self.IJ[i] == node{
					let ind = self.IJ.len() - 1 - i;
					let n = self.IJ[ind];
					self.dfs(n, color);
				}
			}
		}
	}
}