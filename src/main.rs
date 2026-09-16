use clap::Parser;
use skill_fence::{assess, probe_redblue, probe_ruvector};
use std::{fs, path::PathBuf};

#[derive(Debug, Parser)]
#[command(about = "Local-first release evidence for AI agent skills")]
struct Args {
    /// Directory containing SKILL.md.
    #[arg(long)]
    skill: PathBuf,
    /// Other skill directories to compare for deterministic duplicate signals.
    #[arg(long = "compare")]
    comparisons: Vec<PathBuf>,
    /// Write a JSON evidence pack to this path.
    #[arg(long)]
    out: Option<PathBuf>,
    /// Run the real ruvector CLI probe. Its failure is recorded; it never passes a release.
    #[arg(long)]
    ruvector_probe: bool,
    /// Run the real MetaHarness mock red/blue probe. Its failure is recorded; it never passes a release.
    #[arg(long)]
    redblue_probe: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut assessment = assess(&args.skill, &args.comparisons)?;
    if args.ruvector_probe {
        assessment
            .external_evidence
            .retain(|e| e.adapter != "ruvector");
        assessment
            .external_evidence
            .push(probe_ruvector("agent skill release evidence"));
    }
    if args.redblue_probe {
        assessment
            .external_evidence
            .retain(|e| e.adapter != "metaharness-redblue");
        assessment.external_evidence.push(probe_redblue());
    }
    let json = serde_json::to_string_pretty(&assessment)?;
    if let Some(path) = args.out {
        fs::write(path, json)?;
    } else {
        println!("{json}");
    }
    eprintln!("{}", assessment.markdown());
    if assessment.releaseable() {
        Ok(())
    } else {
        std::process::exit(2)
    }
}
