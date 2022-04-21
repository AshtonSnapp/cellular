//! This module exists to help you create 3D (aka flat) cellular automata.
//! 
//! To start, you'll want to decide on your rules and create an AutomataRules object containing them.
//! 
//! ```
//! let rules = AutomataRules::new(Rule::Single(4), Rule::Single(4), 5, Method::Moore);
//! ```
//! 
//! 

//--> Imports <--

use crate::{AutomataRules, Method, Rule};
use std::hash::Hash;
use std::ops::{Add, Sub};
use std::default::Default;
use std::collections::HashMap;

//--> Structs <--

/// A position on a 3D grid, or the size of a 3D grid.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Vec3 { x: usize, y: usize, z: usize }

/// The humble 3D cellular automaton.
pub struct Automaton {
	rules: AutomataRules,
	bounds: Vec3,
	cells: HashMap<Vec3, u8>
}

//--> Functions <--

impl Vec3 {
	/// Creates a new 3D position.
	pub fn new(x: usize, y: usize, z: usize) -> Vec3 { Vec3 { x, y, z } }
}

impl Add for Vec3 {
	type Output = Vec3;
	fn add(self, rhs: Vec3) -> Vec3 {
		Vec3 {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z
		}
	}
}

impl Sub for Vec3 {
	type Output = Vec3;
	fn sub(self, rhs: Vec3) -> Vec3 {
		Vec3 {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z
		}
	}
}

impl Default for Vec3 {
	fn default() -> Vec3 { Vec3 { x: 0, y: 0, z: 0 } }
}

impl Automaton {
	/// Creates a new deep (3D) automaton with the given rules, bounds, and starting cells.
	/// This can fail if your survival and birth rules exceeds the amount of neighbors a cell could have, given your chosen neighbor counting method.
	/// If that happens, this function will error out and return the maximum amount of neighbors.
	pub fn new(rules: AutomataRules, bounds: Vec3, start_cells: Vec<Vec3>) -> Result<Automaton, u8> {
		let other_rules = rules.clone();
		let mut a = Automaton { rules, bounds, cells: HashMap::new() };

		let max_neighbors: u8 = match a.rules.neighbor_method {
			Method::Moore => 26,
			Method::VonNeumann => 6
		};

		match other_rules.to_survive {
			Rule::Single(s) => if s > max_neighbors { return Err(max_neighbors) },
			Rule::Range(r) => if r.start > max_neighbors || r.end > max_neighbors { return Err(max_neighbors) },
			Rule::Many(m) => for s in m {
				if s > max_neighbors { return Err(max_neighbors) }
			}
		}

		match other_rules.to_be_born {
			Rule::Single(s) => if s > max_neighbors { return Err(max_neighbors) },
			Rule::Range(r) => if r.start > max_neighbors || r.end > max_neighbors { return Err(max_neighbors) },
			Rule::Many(m) => for s in m {
				if s > max_neighbors { return Err(max_neighbors) }
			}
		}

		for x in 0..a.bounds.x {
			for y in 0..a.bounds.y {
				for z in 0..a.bounds.z {
					let v = Vec3::new(x, y, z);

					if start_cells.contains(&v) {
						a.cells.insert(v, a.rules.cell_states - 1);
					} else {
						a.cells.insert(v, 0);
					}
				}
			}
		}

		Ok(a)
	}

	/// Advances the automaton by one time step (or tick).
	pub fn tick(&mut self) {
		let neighbor_counts = self.cells.iter().map(|(v, _)| {
			let mut count = 0;
			let mut poss_neighbors = Vec::new();

			// primary directions (up, down, left, right, front, back)

			// only modify x-axis
			poss_neighbors.push(Vec3::new(v.x - 1, v.y, v.z));
			poss_neighbors.push(Vec3::new(v.x + 1, v.y, v.z));

			// only modify y-axis
			poss_neighbors.push(Vec3::new(v.x, v.y - 1, v.z));
			poss_neighbors.push(Vec3::new(v.x, v.y + 1, v.z));

			// only modify z-axis
			poss_neighbors.push(Vec3::new(v.x, v.y, v.z - 1));
			poss_neighbors.push(Vec3::new(v.x, v.y, v.z + 1));

			// secondary directions if using Moore
			if let Method::Moore = self.rules.neighbor_method {
				// only keep x-axis
				poss_neighbors.push(Vec3::new(v.x, v.y - 1, v.z - 1));
				poss_neighbors.push(Vec3::new(v.x, v.y - 1, v.z + 1));
				poss_neighbors.push(Vec3::new(v.x, v.y + 1, v.z - 1));
				poss_neighbors.push(Vec3::new(v.x, v.y + 1, v.z + 1));

				// only keep y-axis
				poss_neighbors.push(Vec3::new(v.x - 1, v.y, v.z - 1));
				poss_neighbors.push(Vec3::new(v.x - 1, v.y, v.z + 1));
				poss_neighbors.push(Vec3::new(v.x + 1, v.y, v.z - 1));
				poss_neighbors.push(Vec3::new(v.x + 1, v.y, v.z + 1));

				// only keep z-axis
				poss_neighbors.push(Vec3::new(v.x - 1, v.y - 1, v.z));
				poss_neighbors.push(Vec3::new(v.x - 1, v.y + 1, v.z));
				poss_neighbors.push(Vec3::new(v.x + 1, v.y - 1, v.z));
				poss_neighbors.push(Vec3::new(v.x + 1, v.y + 1, v.z));

				// change all axes
				poss_neighbors.push(Vec3::new(v.x - 1, v.y - 1, v.z - 1));
				poss_neighbors.push(Vec3::new(v.x - 1, v.y - 1, v.z + 1));
				poss_neighbors.push(Vec3::new(v.x - 1, v.y + 1, v.z - 1));
				poss_neighbors.push(Vec3::new(v.x - 1, v.y + 1, v.z + 1));
				poss_neighbors.push(Vec3::new(v.x + 1, v.y - 1, v.z - 1));
				poss_neighbors.push(Vec3::new(v.x + 1, v.y - 1, v.z + 1));
				poss_neighbors.push(Vec3::new(v.x + 1, v.y + 1, v.z - 1));
				poss_neighbors.push(Vec3::new(v.x + 1, v.y + 1, v.z + 1));
			}

			for poss_neighbor in poss_neighbors {
				if let Some(s) = self.cells.get(&poss_neighbor) {
					if s > &0 {
						count += 1;
					}
				}
			}

			(v.clone(), count)
		}).collect::<HashMap<Vec3, u8>>();

		self.cells.iter_mut().for_each(|(v, s)| {
			if s == &0 {
				// cell is dead
				match self.rules.to_be_born {
					Rule::Single(ref goal) => {
						if let Some(neighbor_count) = neighbor_counts.get(v) {
							if neighbor_count == goal {
								// cell will be born
								*s = self.rules.cell_states - 1;
							}
						}
					},
					Rule::Range(ref goal_range) => {
						if let Some(neighbor_count) = neighbor_counts.get(v) {
							if goal_range.contains(neighbor_count) {
								// cell will be born
								*s = self.rules.cell_states - 1;
							}
						}
					},
					Rule::Many(ref goals) => {
						if let Some(neighbor_count) = neighbor_counts.get(v) {
							if goals.contains(neighbor_count) {
								// cell will be born
								*s = self.rules.cell_states - 1;
							}
						}
					}
				}
			} else if s == &(self.rules.cell_states - 1) {
				// cell is alive
				match self.rules.to_survive {
					Rule::Single(ref goal) => {
						if let Some(neighbor_count) = neighbor_counts.get(v) {
							if neighbor_count != goal {
								// cell will start dying now
								*s -= 1;
							}
						} else {
							// cell should not exist
							*s = 0;
						}
					},
					Rule::Range(ref goal_range) => {
						if let Some(neighbor_count) = neighbor_counts.get(v) {
							if !goal_range.contains(neighbor_count) {
								// cell will start dying now
								*s -= 1;
							}
						} else {
							// cell should not exist
							*s = 0;
						}
					},
					Rule::Many(ref goals) => {
						if let Some(neighbor_count) = neighbor_counts.get(v) {
							if !goals.contains(neighbor_count) {
								// cell will start dying now
								*s -= 1;
							}
						} else {
							// cell should not exist
							*s = 0;
						}
					}
				}
			} else {
				// cell is dying
				*s -= 1;
			}
		});
	}

	/// Get a copy of the automaton's internal state (the cells).
	pub fn get_cells(&self) -> HashMap<Vec3, u8> {
		self.cells.clone()
	}
}