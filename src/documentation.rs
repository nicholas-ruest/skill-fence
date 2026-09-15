use std::{
    fs,
    path::{Component, Path, PathBuf},
};

const REQUIRED_ADRS: usize = 24;
const REQUIRED_DDD_ARTIFACTS: usize = 12;
const ADR_SECTIONS: [&str; 6] = [
    "Context",
    "Decision",
    "Alternatives Considered",
    "Consequences",
    "Verification",
    "Implementation and Domain Links",
];
const DDD_SECTIONS: [&str; 4] = [
    "Bounded context",
    "Ubiquitous language",
    "Invariants",
    "Traceability",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentationReport {
    pub accepted_adrs: usize,
    pub ddd_artifacts: usize,
}

trait DocumentationSource {
    fn markdown_files(&self, directory: &Path) -> Result<Vec<PathBuf>, String>;
    fn read(&self, path: &Path) -> Result<String, String>;
    fn exists(&self, path: &Path) -> bool;
}

struct FileSystemDocumentationSource;

impl DocumentationSource for FileSystemDocumentationSource {
    fn markdown_files(&self, directory: &Path) -> Result<Vec<PathBuf>, String> {
        let entries = fs::read_dir(directory)
            .map_err(|error| format!("could not list `{}`: {error}", directory.display()))?;
        let mut paths = entries
            .filter_map(|entry| entry.ok().map(|entry| entry.path()))
            .filter(|path| path.extension().is_some_and(|extension| extension == "md"))
            .collect::<Vec<_>>();
        paths.sort();
        Ok(paths)
    }

    fn read(&self, path: &Path) -> Result<String, String> {
        fs::read_to_string(path)
            .map_err(|error| format!("could not read `{}`: {error}", path.display()))
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }
}

pub fn validate_architecture_documentation(
    root: impl AsRef<Path>,
    require_completion: bool,
) -> Result<DocumentationReport, Vec<String>> {
    validate_with_source(
        root.as_ref(),
        require_completion,
        &FileSystemDocumentationSource,
    )
}

fn validate_with_source(
    root: &Path,
    require_completion: bool,
    source: &impl DocumentationSource,
) -> Result<DocumentationReport, Vec<String>> {
    let adr_directory = root.join("docs/adr");
    let ddd_directory = root.join("docs/ddd");
    let adr_paths = source
        .markdown_files(&adr_directory)
        .map_err(|error| vec![error])?;
    let ddd_paths = source
        .markdown_files(&ddd_directory)
        .map_err(|error| vec![error])?;
    let mut errors = Vec::new();
    let mut accepted_adrs = 0;

    for path in &adr_paths {
        match source.read(path) {
            Ok(markdown) => {
                if markdown
                    .lines()
                    .any(|line| line.trim() == "Status: Accepted")
                {
                    accepted_adrs += 1;
                } else {
                    errors.push(format!(
                        "{}: missing `Status: Accepted`",
                        display(root, path)
                    ));
                }
                validate_sections(root, path, &markdown, &ADR_SECTIONS, &mut errors);
                validate_links(root, path, &markdown, source, &mut errors);
                require_link(
                    root,
                    path,
                    &markdown,
                    "/src/",
                    "implementation",
                    &mut errors,
                );
                require_link(root, path, &markdown, "/tests/", "test", &mut errors);
                require_link(
                    root,
                    path,
                    &markdown,
                    "../ddd/",
                    "DDD artifact",
                    &mut errors,
                );
            }
            Err(error) => errors.push(error),
        }
    }

    for path in &ddd_paths {
        match source.read(path) {
            Ok(markdown) => {
                validate_sections(root, path, &markdown, &DDD_SECTIONS, &mut errors);
                validate_links(root, path, &markdown, source, &mut errors);
                require_link(root, path, &markdown, "../adr/", "ADR", &mut errors);
                require_link(
                    root,
                    path,
                    &markdown,
                    "/src/",
                    "implementation",
                    &mut errors,
                );
                require_link(root, path, &markdown, "/tests/", "test", &mut errors);
            }
            Err(error) => errors.push(error),
        }
    }

    let report = DocumentationReport {
        accepted_adrs,
        ddd_artifacts: ddd_paths.len(),
    };
    if require_completion {
        if report.accepted_adrs < REQUIRED_ADRS {
            errors.push(format!(
                "completion gate unmet — accepted ADRs: {}/{}",
                report.accepted_adrs, REQUIRED_ADRS
            ));
        }
        if report.ddd_artifacts < REQUIRED_DDD_ARTIFACTS {
            errors.push(format!(
                "completion gate unmet — DDD artifacts: {}/{}",
                report.ddd_artifacts, REQUIRED_DDD_ARTIFACTS
            ));
        }
    }

    if errors.is_empty() {
        Ok(report)
    } else {
        Err(errors)
    }
}

fn validate_sections(
    root: &Path,
    path: &Path,
    markdown: &str,
    required: &[&str],
    errors: &mut Vec<String>,
) {
    for section in required {
        let heading = format!("## {section}");
        if !markdown.lines().any(|line| line.trim() == heading) {
            errors.push(format!(
                "{}: missing `{heading}` section",
                display(root, path)
            ));
        }
    }
}

fn validate_links(
    root: &Path,
    path: &Path,
    markdown: &str,
    source: &impl DocumentationSource,
    errors: &mut Vec<String>,
) {
    for link in markdown_links(markdown) {
        let target = link.split('#').next().unwrap_or_default();
        if target.is_empty()
            || target.starts_with("http://")
            || target.starts_with("https://")
            || target.starts_with("mailto:")
        {
            continue;
        }
        let resolved = path.parent().unwrap_or(root).join(target);
        if !is_within_root(root, &resolved) {
            errors.push(format!(
                "{}: link target `{target}` escapes the repository root",
                display(root, path)
            ));
            continue;
        }
        if !source.exists(&resolved) {
            errors.push(format!(
                "{}: link target `{target}` does not exist",
                display(root, path)
            ));
        }
    }
}

fn is_within_root(root: &Path, path: &Path) -> bool {
    let Ok(relative) = path.strip_prefix(root) else {
        return false;
    };
    let mut depth = 0usize;
    for component in relative.components() {
        match component {
            Component::Normal(_) => depth += 1,
            Component::CurDir => {}
            Component::ParentDir if depth > 0 => depth -= 1,
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => return false,
        }
    }
    true
}

fn require_link(
    root: &Path,
    path: &Path,
    markdown: &str,
    fragment: &str,
    label: &str,
    errors: &mut Vec<String>,
) {
    if !markdown_links(markdown)
        .iter()
        .any(|link| link.contains(fragment))
    {
        errors.push(format!(
            "{}: missing {label} traceability link",
            display(root, path)
        ));
    }
}

fn markdown_links(markdown: &str) -> Vec<&str> {
    let mut remainder = markdown;
    let mut links = Vec::new();
    while let Some(start) = remainder.find("](") {
        remainder = &remainder[start + 2..];
        let Some(end) = remainder.find(')') else {
            break;
        };
        links.push(&remainder[..end]);
        remainder = &remainder[end + 1..];
    }
    links
}

fn display(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .display()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::RefCell, collections::BTreeMap};

    struct RecordingDocumentationSource {
        documents: BTreeMap<PathBuf, String>,
        reads: RefCell<Vec<PathBuf>>,
    }

    impl DocumentationSource for RecordingDocumentationSource {
        fn markdown_files(&self, directory: &Path) -> Result<Vec<PathBuf>, String> {
            Ok(self
                .documents
                .keys()
                .filter(|path| path.parent() == Some(directory))
                .cloned()
                .collect())
        }

        fn read(&self, path: &Path) -> Result<String, String> {
            self.reads.borrow_mut().push(path.to_path_buf());
            self.documents
                .get(path)
                .cloned()
                .ok_or_else(|| format!("missing {}", path.display()))
        }

        fn exists(&self, _path: &Path) -> bool {
            true
        }
    }

    #[test]
    fn requests_every_architecture_document_through_the_owned_source() {
        let root = PathBuf::from("repository");
        let adr = root.join("docs/adr/ADR-001.md");
        let ddd = root.join("docs/ddd/context.md");
        let adr_markdown = format!(
            "# ADR\n\nStatus: Accepted\n\n{}\n\n[code](../../src/lib.rs) [test](../../tests/check.rs) [domain](../ddd/context.md)",
            ADR_SECTIONS
                .iter()
                .map(|section| format!("## {section}"))
                .collect::<Vec<_>>()
                .join("\n\n")
        );
        let ddd_markdown = format!(
            "# Context\n\n{}\n\n[decision](../adr/ADR-001.md) [code](../../src/lib.rs) [test](../../tests/check.rs)",
            DDD_SECTIONS
                .iter()
                .map(|section| format!("## {section}"))
                .collect::<Vec<_>>()
                .join("\n\n")
        );
        let source = RecordingDocumentationSource {
            documents: BTreeMap::from([(adr.clone(), adr_markdown), (ddd.clone(), ddd_markdown)]),
            reads: RefCell::new(Vec::new()),
        };

        let report = validate_with_source(&root, false, &source).expect("valid documentation");

        assert_eq!(report.accepted_adrs, 1);
        assert_eq!(report.ddd_artifacts, 1);
        assert_eq!(*source.reads.borrow(), vec![adr, ddd]);
    }

    #[test]
    fn rejects_an_adr_when_test_traceability_is_removed() {
        let root = PathBuf::from("repository");
        let adr = root.join("docs/adr/ADR-001.md");
        let ddd = root.join("docs/ddd/context.md");
        let adr_markdown = format!(
            "# ADR\n\nStatus: Accepted\n\n{}\n\n[code](../../src/lib.rs) [domain](../ddd/context.md)",
            ADR_SECTIONS
                .iter()
                .map(|section| format!("## {section}"))
                .collect::<Vec<_>>()
                .join("\n\n")
        );
        let ddd_markdown = format!(
            "# Context\n\n{}\n\n[decision](../adr/ADR-001.md) [code](../../src/lib.rs) [test](../../tests/check.rs)",
            DDD_SECTIONS
                .iter()
                .map(|section| format!("## {section}"))
                .collect::<Vec<_>>()
                .join("\n\n")
        );
        let source = RecordingDocumentationSource {
            documents: BTreeMap::from([(adr, adr_markdown), (ddd, ddd_markdown)]),
            reads: RefCell::new(Vec::new()),
        };

        let errors = validate_with_source(&root, false, &source)
            .expect_err("an ADR without test traceability must fail");

        assert!(
            errors
                .iter()
                .any(|error| error.contains("missing test traceability link")),
            "{errors:?}"
        );
    }

    #[test]
    fn rejects_traceability_that_escapes_the_repository() {
        let root = PathBuf::from("repository");
        let adr = root.join("docs/adr/ADR-001.md");
        let ddd = root.join("docs/ddd/context.md");
        let adr_markdown = format!(
            "# ADR\n\nStatus: Accepted\n\n{}\n\n[code](../../src/lib.rs) [test](../../../../outside/tests/check.rs) [domain](../ddd/context.md)",
            ADR_SECTIONS
                .iter()
                .map(|section| format!("## {section}"))
                .collect::<Vec<_>>()
                .join("\n\n")
        );
        let ddd_markdown = format!(
            "# Context\n\n{}\n\n[decision](../adr/ADR-001.md) [code](../../src/lib.rs) [test](../../tests/check.rs)",
            DDD_SECTIONS
                .iter()
                .map(|section| format!("## {section}"))
                .collect::<Vec<_>>()
                .join("\n\n")
        );
        let source = RecordingDocumentationSource {
            documents: BTreeMap::from([(adr, adr_markdown), (ddd, ddd_markdown)]),
            reads: RefCell::new(Vec::new()),
        };

        let errors = validate_with_source(&root, false, &source)
            .expect_err("repository-escaping traceability must fail");

        assert!(
            errors
                .iter()
                .any(|error| error.contains("escapes the repository root")),
            "{errors:?}"
        );
    }
}
