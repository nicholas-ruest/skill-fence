use clap::Parser;
use skill_fence::validate_architecture_documentation;
use std::{path::PathBuf, process::ExitCode};

#[derive(Debug, Parser)]
#[command(about = "Validate SkillFence ADR/DDD schema and traceability")]
struct Args {
    /// Repository root containing docs/adr and docs/ddd.
    #[arg(long, default_value = ".")]
    root: PathBuf,
    /// Also enforce the Product Foundry completion count gates.
    #[arg(long)]
    completion: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();
    match validate_architecture_documentation(&args.root, args.completion) {
        Ok(report) => {
            println!(
                "architecture documentation valid — accepted ADRs: {}; DDD artifacts: {}",
                report.accepted_adrs, report.ddd_artifacts
            );
            ExitCode::SUCCESS
        }
        Err(errors) => {
            for error in errors {
                eprintln!("{error}");
            }
            ExitCode::FAILURE
        }
    }
}
