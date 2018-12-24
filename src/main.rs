mod parser;
mod solver;
mod types;

use std::io;

fn main() {
    let specification = parser::parse(io::stdin().lock());
    let solution = solver::solve(specification);

    match solution {
        Some(output) => println!("{}", output),
        None => println!("No solution exists"),
    };
}
