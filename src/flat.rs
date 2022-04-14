//! This module exists to help you create 2D (aka flat) cellular automata.
//! 
//! To start, you'll want to decide on your rules and create an AutomataRules object containing them.
//! 
//! ```
//! let rules = AutomataRules::new(Rule::Range(3..5), Rule::Single(3), 2, Method::Moore);
//! ```
//! 
//! If you didn't know, those are the rules for Conway's Game of Life. Anyways, now we'll want to decide on our starting state, or seed.
//! This will be a vector of 2-component vectors.
//! 
//! ```
//! let seed = vec![Vec2::new(1, 0), Vec2::new(2, 1), Vec2::new(0, 2), Vec2::new(1, 2), Vec2::new(2, 2)];
//! ```
//! 
//! And that seed will create a single glider. Now we can create our automata, while also deciding on our bounds (the size of the grid).
//! After all, we don't have infinite memory.
//! 
//! ```
//! let mut life = Automaton::new(rules, Vec2::new(50, 50), seed);
//! ```
//! 
//! Now, you have a cellular automaton running Conway's Game of Life. You can advance the automaton by calling `life.tick()` (or `life.par_tick()` if you have rayon), and get the current internal state by calling `life.get_cells()`.

//--> Imports <--

use crate::{AutomataRules, Method, Rule};
use std::hash::Hash;
use std::ops::{Add, Sub};
use std::default::Default;
use std::collections::HashMap;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

//--> Structs <--

/// A position on a 2D grid, or the size of a 2D grid.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Vec2 { x: usize, y: usize }

/// The humble 2D cellular automaton.
pub struct Automaton {
	rules: AutomataRules,
	bounds: Vec2,
	cells: HashMap<Vec2, u8>
}

//--> Functions <--

impl Vec2 {
	/// Creates a new 2D position.
	pub fn new(x: usize, y: usize) -> Vec2 { Vec2 { x, y } }
}

impl Add for Vec2 {
	type Output = Vec2;
	fn add(self, rhs: Vec2) -> Vec2 {
		Vec2 {
			x: self.x + rhs.x,
			y: self.y + rhs.y
		}
	}
}

impl Sub for Vec2 {
	type Output = Vec2;
	fn sub(self, rhs: Vec2) -> Vec2 {
		Vec2 {
			x: self.x - rhs.x,
			y: self.y - rhs.y
		}
	}
}

impl Default for Vec2 {
	fn default() -> Vec2 { Vec2 { x: 0, y: 0 } }
}

impl Automaton {
	/// Creates a new flat (2D) automaton with the given rules, bounds, and starting cells.
	/// This can fail if your survival and birth rules exceeds the amount of neighbors a cell could have, given your chosen neighbor counting method.
	/// If that happens, this function will error out and return the maximum amount of neighbors.
	pub fn new(rules: AutomataRules, bounds: Vec2, start_cells: Vec<Vec2>) -> Result<Automaton, u8> {
		let other_rules = rules.clone();
		let mut a = Automaton { rules, bounds, cells: HashMap::new() };

		let max_neighbors: u8 = match a.rules.neighbor_method {
			Method::Moore => 8,
			Method::VonNeumann => 4
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
				let v = Vec2::new(x, y);

				if start_cells.contains(&v) {
					a.cells.insert(v, a.rules.cell_states - 1);
				} else {
					a.cells.insert(v, 0);
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

			// primary directions (up, down, left, right)
			poss_neighbors.push(Vec2::new(v.x, v.y - 1));
			poss_neighbors.push(Vec2::new(v.x, v.y + 1));
			poss_neighbors.push(Vec2::new(v.x - 1, v.y));
			poss_neighbors.push(Vec2::new(v.x + 1, v.y));

			// secondary directions (up-left, up-right, down-left, down-right) if using Moore
			if let Method::Moore = self.rules.neighbor_method {
				poss_neighbors.push(Vec2::new(v.x - 1, v.y - 1));
				poss_neighbors.push(Vec2::new(v.x - 1, v.y + 1));
				poss_neighbors.push(Vec2::new(v.x + 1, v.y - 1));
				poss_neighbors.push(Vec2::new(v.x + 1, v.y + 1));
			}

			for poss_neighbor in poss_neighbors {
				if let Some(s) = self.cells.get(&poss_neighbor) {
					if s > &0 {
						count += 1;
					}
				}
			}

			(v.clone(), count)
		}).collect::<HashMap<Vec2, u8>>();

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

	/// Advances the automaton by one time step (or tick), but using multiple threads.
	#[cfg(feature = "rayon")]
	pub fn par_tick(&mut self) {
		let neighbor_counts = self.cells.par_iter().map(|(v, _)| {
			let mut count = 0;
			let mut poss_neighbors = Vec::new();

			// primary directions (up, down, left, right)
			poss_neighbors.push(Vec2::new(v.x, v.y - 1));
			poss_neighbors.push(Vec2::new(v.x, v.y + 1));
			poss_neighbors.push(Vec2::new(v.x - 1, v.y));
			poss_neighbors.push(Vec2::new(v.x + 1, v.y));

			// secondary directions (up-left, up-right, down-left, down-right) if using Moore
			if let Method::Moore = self.rules.neighbor_method {
				poss_neighbors.push(Vec2::new(v.x - 1, v.y - 1));
				poss_neighbors.push(Vec2::new(v.x - 1, v.y + 1));
				poss_neighbors.push(Vec2::new(v.x + 1, v.y - 1));
				poss_neighbors.push(Vec2::new(v.x + 1, v.y + 1));
			}

			for poss_neighbor in poss_neighbors {
				if let Some(s) = self.cells.get(&poss_neighbor) {
					if s > &0 {
						count += 1;
					}
				}
			}

			(v.clone(), count)
		}).collect::<HashMap<Vec2, u8>>();

		self.cells.par_iter_mut().for_each(|(v, s)| {
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
	pub fn get_cells(&self) -> HashMap<Vec2, u8> {
		self.cells.clone()
	}
}