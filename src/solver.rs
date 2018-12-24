use crate::types::*;
use std::collections::BinaryHeap;

pub fn solve(spec: Specification) -> Option<Solution> {
    // By using a min heap with accumulated score to queue next
    // node to extend the search to (best-first approach) we explore
    // lowest code solutions first

    let mut heap: BinaryHeap<Solution> = BinaryHeap::new();

    heap.push(Solution::new());

    while !heap.is_empty() {
        let solution = heap.pop().unwrap();

        // Check if we can prune this entire subtree based on constraints

        if solution.violates_contraints(&spec) {
            continue;
        }

        // If leaf node met, stop recursing

        if solution.len() == spec.num_colors {
            // Check for complete solution and stop if found
            // is guaranteed to be lowest cost solution possible

            if solution.complete(&spec) {
                return Some(solution);
            }

            continue;
        }

        heap.push(solution.extend(Finish::Matte));
        heap.push(solution.extend(Finish::Gloss));
    }

    None
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;
    use std::io::BufReader;

    use crate::parser::parse;
    use crate::solver::solve;
    use crate::types::*;

    #[test]
    fn it_finds_solution() {
        let mut essentials = HashMap::new();
        essentials.insert(1, Finish::Matte);
        essentials.insert(3, Finish::Matte);

        let mut pref1 = HashMap::new();
        pref1.insert(1, Finish::Matte);

        let mut pref2 = HashMap::new();
        pref2.insert(2, Finish::Gloss);
        pref2.insert(1, Finish::Gloss);

        let mut pref3 = HashMap::new();
        pref3.insert(3, Finish::Matte);

        let preferences = vec![pref1, pref2, pref3];

        let spec = Specification {
            num_colors: 3,
            essentials,
            preferences,
        };

        let solution = solve(spec);

        assert_eq!(
            solution.unwrap(),
            Solution {
                cost: 2,
                choices: vec![Finish::Matte, Finish::Gloss, Finish::Matte,]
            }
        )
    }

    #[test]
    fn it_finds_no_solution() {
        let input = "1\n1 M\n1 G".as_bytes();
        let spec = parse(BufReader::new(input));
        let solution = solve(spec);

        assert!(solution.is_none())
    }
}
