#[allow(non_snake_case)]
pub struct Graph {
	pub IJ: Vec<i32>,
	pub H: Vec<i32>,
	pub L: Vec<i32>,
	pub arcs_count: i32,
	pub weights: Vec<i32>,
	pub compSzyaznDFS: Vec<i32>,
}

impl Graph {
	pub fn new() -> Graph{
		Graph{
			IJ: Vec::new(),
			H: Vec::new(),
			L: Vec::new(),
			arcs_count: 0,
			weights: Vec::new(),
			compSzyaznDFS: Vec::new(),
		}
	}

	pub fn fill(& mut self, ac: i32, vi: Vec<i32>, vj: Vec<i32>){
		self.arcs_count = ac;

		for i in 0..vi.len() {
			self.IJ.push(vi[i]);
		}

		for i in 0..vj.len() {
			self.IJ.push(vj[vj.len() - 1 - i]);
		}

		for _i in 0..ac {
			self.H.push(-1);
		}

		for _i in 0..self.IJ.len() {
			self.L.push(-1);
		}

		for _i in 0..ac {
			self.weights.push(-1);
		}

		for _i in 0..self.arcs_count {
			self.compSzyaznDFS.push(-1);
		}

		self.compute_HL();
	}

	#[allow(non_snake_case)]
	fn compute_HL(& mut self){
		for i in 0..self.IJ.len(){
			let k = self.IJ[i];
			self.L[i] = self.H[k as usize];
			self.H[k as usize] = i as i32;
		}
	}
}

use std::fmt;

impl fmt::Display for Graph{
	fn fmt(&self, f: & mut fmt::Formatter) -> fmt::Result{
		let _ = writeln!(f, "IJ({}): {:?}", self.IJ.len(), self.IJ);
		let _ = writeln!(f, "H({}): {:?}", self.H.len(), self.H);
		let _ = writeln!(f, "L({}): {:?}", self.L.len(), self.L);
		let _ = writeln!(f, "weights({}): {:?}", self.weights.len(), self.weights);
		let _ = writeln!(f, "compSzyaznDFS({}): {:?}", self.compSzyaznDFS.len(), self.compSzyaznDFS);
		Ok(())
	}
}