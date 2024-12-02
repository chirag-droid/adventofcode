use clap::{Parser, Subcommand};
use anyhow::Result;

mod registry;
mod generator;
mod solutions;

use registry::SolutionManager;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
   #[command(subcommand)]
   command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
   /// Run a specific day's solution
   Run {
      /// Day number to run (e.g., 1, 2, 3...)
      #[arg(value_parser = clap::value_parser!(u32))]
      day: u32,

      /// Part of the problem to solve (1 or 2)
      #[arg(value_parser = clap::value_parser!(u32))]
      part: u32,
   },

   /// List all available solutions
   List,

   /// Generate a new solution template
   Generate {
      /// Day number to generate (e.g., 1, 2, 3...)
      #[arg(value_parser = clap::value_parser!(u32))]
      day: u32,
   },
}

fn main() -> Result<()> {
   let cli = Cli::parse();

   match &cli.command {
      Some(Commands::Run { day, part }) => {
         if let Some(solution) = SolutionManager::get(*day) {
               solution(*part)?;
         } else {
               println!("Solution for day {} not found", day);
               std::process::exit(1);
         }
      },
      Some(Commands::List) => {
         println!("Available Solutions:");
         for day in SolutionManager::list_days() {
            println!("- Day {}", day);
         }
      },
      Some(Commands::Generate { day }) => {
         generator::generate_solution(*day)?;
      },
      None => {
         println!("No command specified. Use --help to see available commands.");
      }
   }

   Ok(())
}
