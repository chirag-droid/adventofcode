use anyhow::Result;
use crate::register_solution;

pub fn solve(part: u32) -> Result<()> {
    match part {
        1 => solve_part1(),
        2 => solve_part2(),
        _ => {
            println!("Invalid part number");
            Ok(())
        }
    }
}

fn solve_part1() -> Result<()> {
    println!("Day 1 Part 1 Solution");
    // Implement part 1 logic here
    Ok(())
}

fn solve_part2() -> Result<()> {
    println!("Day 1 Part 2 Solution");
    // Implement part 2 logic here
    Ok(())
}

// Register the solution
register_solution!(1, solve);
