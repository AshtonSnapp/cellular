//! Create cellular automata.
//! 
//! Inspired by tantan's 3D cellular automata project, the cellular library aims to allow you to create and simulate 2D and 3D cellular automata.
//! Unlike tantan's project, this isn't bound to any one framework or game engine for displaying the automata graphically.
//! This just does the simulation bits.

//--> Imports <--

use std::ops::Range;

/// Create flat (2D) cellular automata.
pub mod flat;

/// Create deep (3D) cellular automata.
pub mod deep;

//--> Structs <--

/// Any cellular automata has four different rules that govern how it works.
/// 
/// Any given cell has a certain amount of neighbors. Whether any cell is a neighbor of said cell is up to the neighbor method, which is either Moore or Von Neumann.
/// 
/// A cell must have a certain amount of neighbors to either become alive or stay alive.
/// 
/// Finally, cells in an automaton have a certain amount of 'states'. Live cells have a state equal to whatever value you put in here, minus 1.
/// 
/// You can think of these states as the amount of time steps (or ticks) it takes for a cell to die.
/// Given a cell which is alive but no longer has enough neighbors to survive, its state value will be decremented to 0 (dead) each tick.
#[derive(Clone)]
pub struct AutomataRules {
    to_survive: Rule,
    to_be_born: Rule,
    cell_states: u8,
    neighbor_method: Method
}

//--> Enums <--

/// Any cellular automata has two rules that care about neighbors.
/// One rule says how many neighbors a cell needs to be born, and another says how many neighbors a cell needs to continue living.
/// This enum is used to tell the automata how many neighbor counts are valid for a given rule.
#[derive(Clone)]
pub enum Rule {
    /// This rule only matches a single count of neighbors.
    Single(u8),
    /// This rule matches a consecutive set of neighbor counts.
    /// Note that this range is inclusive on the start and exclusive on the end, so a range of 3..5 will include the values 3 and 4.
    Range(Range<u8>),
    /// This rule matches a non-consecutive set of neighbor counts.
    Many(Vec<u8>)
}

/// Any cellular automata has one of two ways to determine whether any given cell is the neighbor of any other cell.
/// This enum allows choosing which method is used by an automaton to determine neighbors.
#[derive(Clone)]
pub enum Method {
    /// The Moore method counts any cell as a neighbor of a given cell if that cell is next to it, even if they don't share a face.
    /// More mathmatically, if any two cells have coordinates that are only off by one from each-other for any given component, they are neighbors.
    Moore,
    /// The Von Neumann method counts any cell as a neighbor of a given cell if the cells share a face, or are touching.
    VonNeumann
}

//--> Functions <--

impl AutomataRules {
    /// Creates a new set of cellular automaton rules.
    pub fn new(to_survive: Rule, to_be_born: Rule, cell_states: u8, neighbor_method: Method) -> AutomataRules {
        AutomataRules {
            to_survive,
            to_be_born,
            cell_states,
            neighbor_method
        }
    }
}

//--> Tests <--

#[cfg(test)]
mod tests {}