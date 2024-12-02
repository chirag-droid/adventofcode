use std::fs::{self, File};
use std::io::{Write, Result as IoResult};
use std::path::Path;

pub fn generate_solution(day_number: u32) -> IoResult<()> {
    let module_name = format!("day{:0>2}", day_number);
    let solutions_dir = Path::new("src/solutions");

    // Ensure solutions directory exists
    fs::create_dir_all(solutions_dir)?;

    let solution_content = generate_solution_template(&module_name, day_number);
    
    // Write solution file
    let file_path = solutions_dir.join(format!("{}.rs", module_name));
    let mut file = File::create(&file_path)?;
    file.write_all(solution_content.as_bytes())?;

    // Update mod.rs
    update_mod_rs(&module_name, day_number)?;

    println!("Created solution for Day {}", day_number);
    Ok(())
}

fn generate_solution_template(module_name: &str, day_number: u32) -> String {
    format!(
r#"use anyhow::Result;
use crate::register_solution;

pub fn solve(part: u32) -> Result<()> {{
    match part {{
        1 => solve_part1(),
        2 => solve_part2(),
        _ => {{
            println!("Invalid part number");
            Ok(())
        }}
    }}
}}

fn solve_part1() -> Result<()> {{
    println!("Day {} Part 1 Solution");
    // Implement part 1 logic here
    Ok(())
}}

fn solve_part2() -> Result<()> {{
    println!("Day {} Part 2 Solution");
    // Implement part 2 logic here
    Ok(())
}}

// Register the solution
register_solution!({}, solve);
"#, day_number, day_number, day_number)
}

fn update_mod_rs(module_name: &str, day_number: u32) -> IoResult<()> {
    let mod_path = Path::new("src/solutions/mod.rs");
    
    // Read existing contents or create if not exists
    let mut mod_contents = fs::read_to_string(mod_path)
        .unwrap_or_else(|_| String::new());

    // Add module import if not exists
    if !mod_contents.contains(&format!("pub mod {};", module_name)) {
        // Append module declaration
        mod_contents.push_str(&format!("pub mod {};\n", module_name));
    }

    // Trim to remove any trailing whitespace
    mod_contents = mod_contents.trim().to_string();
    
    fs::write(mod_path, mod_contents)?;

    Ok(())
}
