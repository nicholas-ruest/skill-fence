use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExternalStatus {
    NotRequested,
    Passed,
    Failed,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalEvidence {
    pub adapter: String,
    pub status: ExternalStatus,
    pub command: Vec<String>,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub error: Option<String>,
    pub detail: String,
}

pub(crate) fn ruvector_not_requested() -> ExternalEvidence {
    not_requested(
        "ruvector",
        "ruvector",
        &["embed", "text", "agent skill release evidence"],
        "Run with --ruvector-probe to invoke the installed ruvector CLI.",
    )
}

pub(crate) fn redblue_not_requested() -> ExternalEvidence {
    not_requested(
        "metaharness-redblue",
        "metaharness",
        &["pro", "redblue", "run", "--mock-judge"],
        "Run with --redblue-probe to invoke `metaharness pro redblue run --mock-judge`.",
    )
}

fn not_requested(
    adapter: &str,
    program: &str,
    arguments: &[&str],
    detail: &str,
) -> ExternalEvidence {
    ExternalEvidence {
        adapter: adapter.into(),
        status: ExternalStatus::NotRequested,
        command: command_receipt(program, arguments),
        exit_code: None,
        stdout: String::new(),
        stderr: String::new(),
        error: None,
        detail: detail.into(),
    }
}

pub fn probe_ruvector(text: &str) -> ExternalEvidence {
    probe_ruvector_with(text, &CommandExternalProcess)
}

pub fn probe_redblue() -> ExternalEvidence {
    probe_redblue_with(&CommandExternalProcess)
}

struct ProcessObservation {
    exit_code: Option<i32>,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

trait ExternalProcess {
    fn run(&self, program: &str, arguments: &[&str]) -> std::io::Result<ProcessObservation>;
}

struct CommandExternalProcess;

impl ExternalProcess for CommandExternalProcess {
    fn run(&self, program: &str, arguments: &[&str]) -> std::io::Result<ProcessObservation> {
        Command::new(program)
            .args(arguments)
            .output()
            .map(|output| ProcessObservation {
                exit_code: output.status.code(),
                stdout: output.stdout,
                stderr: output.stderr,
            })
    }
}

fn probe_ruvector_with(text: &str, process: &impl ExternalProcess) -> ExternalEvidence {
    probe("ruvector", "ruvector", &["embed", "text", text], process)
}

fn probe_redblue_with(process: &impl ExternalProcess) -> ExternalEvidence {
    probe(
        "metaharness-redblue",
        "metaharness",
        &["pro", "redblue", "run", "--mock-judge"],
        process,
    )
}

fn probe(
    adapter: &str,
    program: &str,
    arguments: &[&str],
    process: &impl ExternalProcess,
) -> ExternalEvidence {
    let command = command_receipt(program, arguments);
    match process.run(program, arguments) {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
            let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
            let status = if output.exit_code == Some(0) {
                ExternalStatus::Passed
            } else {
                ExternalStatus::Failed
            };
            let detail = if status == ExternalStatus::Passed {
                stdout.trim().to_owned()
            } else {
                stderr.trim().to_owned()
            };
            ExternalEvidence {
                adapter: adapter.into(),
                status,
                command,
                exit_code: output.exit_code,
                stdout,
                stderr,
                error: None,
                detail,
            }
        }
        Err(error) => ExternalEvidence {
            adapter: adapter.into(),
            status: ExternalStatus::Unavailable,
            command,
            exit_code: None,
            stdout: String::new(),
            stderr: String::new(),
            detail: error.to_string(),
            error: Some(error.to_string()),
        },
    }
}

fn command_receipt(program: &str, arguments: &[&str]) -> Vec<String> {
    std::iter::once(program)
        .chain(arguments.iter().copied())
        .map(str::to_owned)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct RecordingExternalProcess {
        requested: RefCell<Vec<(String, Vec<String>)>>,
    }

    impl ExternalProcess for RecordingExternalProcess {
        fn run(&self, program: &str, arguments: &[&str]) -> std::io::Result<ProcessObservation> {
            self.requested.borrow_mut().push((
                program.to_owned(),
                arguments
                    .iter()
                    .map(|argument| (*argument).to_owned())
                    .collect(),
            ));
            Ok(ProcessObservation {
                exit_code: Some(0),
                stdout: b"owned port output\n".to_vec(),
                stderr: Vec::new(),
            })
        }
    }

    #[test]
    fn requests_ruvector_through_the_owned_process_port() {
        let process = RecordingExternalProcess {
            requested: RefCell::new(Vec::new()),
        };

        let receipt = probe_ruvector_with("bounded input", &process);

        assert_eq!(
            *process.requested.borrow(),
            vec![(
                "ruvector".into(),
                vec!["embed".into(), "text".into(), "bounded input".into()]
            )]
        );
        assert_eq!(receipt.status, ExternalStatus::Passed);
        assert_eq!(receipt.exit_code, Some(0));
        assert_eq!(receipt.stdout, "owned port output\n");
        assert_eq!(receipt.stderr, "");
        assert_eq!(receipt.error, None);
    }
}
