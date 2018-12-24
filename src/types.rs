use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

// A paint finish may be Matte or Gloss

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Finish {
    Matte,
    Gloss,
}

// A datastructure that holds one individual customer's preferences

pub type CustomerPreferences = HashMap<usize, Finish>;

// A Specification consists of the number of colors to produce and all of the customers' preferences

#[derive(Debug, PartialEq)]
pub struct Specification {
    pub num_colors: usize,
    pub preferences: Vec<CustomerPreferences>,
    pub essentials: CustomerPreferences,
}

// A potential solution of the solver

#[derive(Debug, PartialEq, Eq)]
pub struct Solution {
    pub choices: Vec<Finish>,
    pub cost: usize,
}

impl Solution {
    pub fn new() -> Solution {
        Solution {
            choices: vec![],
            cost: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.choices.len()
    }

    pub fn extend(&self, finish: Finish) -> Solution {
        let mut choices = self.choices.clone();
        let mut cost = self.cost.clone();

        if finish == Finish::Matte {
            cost += 1;
        }

        choices.push(finish);
        Solution { choices, cost }
    }

    pub fn complete(&self, spec: &Specification) -> bool {
        'outer: for prefereces in &spec.preferences {
            for (paint, finish) in prefereces {
                let candidate = self.choices.get(paint - 1).expect("Missing paint");
                if candidate == finish {
                    continue 'outer;
                }
            }

            return false;
        }

        true
    }

    pub fn violates_contraints(&self, spec: &Specification) -> bool {
        for (key, finish) in &spec.essentials {
            let choice = self.choices.get(*key - 1);
            if let Some(chosen_finish) = choice {
                if chosen_finish != finish {
                    return true;
                }
            }
        }

        false
    }
}

// Implement traits that allow Solution to be inserted into BinaryHeap

impl Ord for Solution {
    fn cmp(&self, other: &Solution) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Solution {
    fn partial_cmp(&self, other: &Solution) -> Option<Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buff = String::new();
        for choice in &self.choices {
            buff.push_str(match choice {
                Finish::Matte => "M ",
                Finish::Gloss => "G ",
            });
        }
        write!(f, "{}", buff.trim())
    }
}
