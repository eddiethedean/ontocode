use crate::error::{Result, RobotError};
use std::io::ErrorKind;
use std::path::{Component, Path};
use std::process::Command;

#[derive(Debug)]
pub struct RobotOutput {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

const ALLOWED_SUBCOMMANDS: &[&str] =
    &["validate-profile", "validate", "merge", "report", "reason", "query", "convert"];

/// Resolve a ROBOT executable. Explicit paths must be named `robot` / `robot.cmd` / `robot.bat`.
pub fn detect_robot(explicit_path: Option<&str>) -> Result<String> {
    if let Some(path) = explicit_path.filter(|p| !p.is_empty()) {
        let p = Path::new(path);
        let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("").to_ascii_lowercase();
        if !matches!(name.as_str(), "robot" | "robot.cmd" | "robot.bat" | "robot.exe") {
            return Err(RobotError::NotFound);
        }
        // Reject path traversal in the explicit path.
        if p.components().any(|c| matches!(c, Component::ParentDir)) {
            return Err(RobotError::NotFound);
        }
        if p.exists() {
            return Ok(path.to_string());
        }
        return Err(RobotError::NotFound);
    }
    // Probe PATH by spawning allowlisted names. Avoids Unix-only `which` (#316);
    // Windows PATHEXT resolves `robot` → `robot.cmd` / `.bat` / `.exe` when present.
    for name in ["robot", "robot.cmd", "robot.bat", "robot.exe"] {
        if robot_exists_on_path(name) {
            return Ok(name.to_string());
        }
    }
    Err(RobotError::NotFound)
}

fn robot_exists_on_path(name: &str) -> bool {
    match Command::new(name).arg("--version").output() {
        Ok(_) => true,
        Err(err) => err.kind() != ErrorKind::NotFound,
    }
}

pub fn run_robot(robot_path: Option<&str>, args: &[String]) -> Result<RobotOutput> {
    if let Some(sub) = args.first() {
        let base = sub.split_whitespace().next().unwrap_or(sub);
        if !ALLOWED_SUBCOMMANDS.contains(&base) {
            return Err(RobotError::Run(format!(
                "ROBOT subcommand '{base}' is not allowed; permitted: {}",
                ALLOWED_SUBCOMMANDS.join(", ")
            )));
        }
    } else {
        return Err(RobotError::Run("ROBOT requires a subcommand".to_string()));
    }
    let robot = detect_robot(robot_path)?;
    let output = Command::new(&robot).args(args).output()?;
    let exit_code = output.status.code().unwrap_or(1);
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    Ok(RobotOutput { exit_code, stdout, stderr })
}

pub fn robot_validate(robot_path: Option<&str>, ontology_path: &Path) -> Result<RobotOutput> {
    run_robot(
        robot_path,
        &[
            "validate-profile".to_string(),
            "--input".to_string(),
            ontology_path.display().to_string(),
            "--profile".to_string(),
            "DL".to_string(),
        ],
    )
}

pub fn robot_convert(robot_path: Option<&str>, input: &Path, output: &Path) -> Result<RobotOutput> {
    run_robot(
        robot_path,
        &[
            "convert".into(),
            "--input".into(),
            input.display().to_string(),
            "--output".into(),
            output.display().to_string(),
        ],
    )
}

pub fn robot_merge(
    robot_path: Option<&str>,
    inputs: &[String],
    output: &Path,
) -> Result<RobotOutput> {
    let mut args = vec!["merge".to_string()];
    for input in inputs {
        args.push("--input".to_string());
        args.push(input.clone());
    }
    args.push("--output".to_string());
    args.push(output.display().to_string());
    run_robot(robot_path, &args)
}

pub fn robot_report(
    robot_path: Option<&str>,
    ontology_path: &Path,
    report_path: &Path,
) -> Result<RobotOutput> {
    run_robot(
        robot_path,
        &[
            "report".to_string(),
            "--input".to_string(),
            ontology_path.display().to_string(),
            "--output".to_string(),
            report_path.display().to_string(),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::RobotError;

    #[test]
    fn rejects_disallowed_robot_subcommand() {
        let err = match run_robot(Some("/definitely/not/robot"), &[String::from("diff")]) {
            Err(e) => e,
            Ok(output) => panic!("expected disallowed subcommand error, got {output:?}"),
        };
        match err {
            RobotError::Run(msg) => {
                assert!(msg.contains("not allowed"));
                assert!(msg.contains("diff"));
            }
            other => panic!("expected Run error, got {other:?}"),
        }
    }

    #[test]
    fn rejects_empty_robot_args() {
        let err = match run_robot(Some("/definitely/not/robot"), &[]) {
            Err(e) => e,
            Ok(output) => panic!("expected empty-args error, got {output:?}"),
        };
        match err {
            RobotError::Run(msg) => assert!(msg.contains("requires a subcommand")),
            other => panic!("expected Run error, got {other:?}"),
        }
    }

    #[test]
    fn allows_known_subcommands_in_validation() {
        for sub in ["validate-profile", "validate", "merge", "report", "reason", "query", "convert"]
        {
            let result = run_robot(Some("/definitely/not/robot"), &[String::from(sub)]);
            match result {
                Err(RobotError::Run(ref msg)) if msg.contains("not allowed") => {
                    panic!("subcommand {sub} should pass allowlist, got {msg}");
                }
                _ => {}
            }
        }
    }

    #[test]
    fn rejects_parent_dir_in_explicit_robot_path() {
        let err = detect_robot(Some("../robot")).unwrap_err();
        assert!(matches!(err, RobotError::NotFound));
    }

    #[test]
    fn rejects_non_robot_binary_name() {
        let err = detect_robot(Some("/usr/bin/java")).unwrap_err();
        assert!(matches!(err, RobotError::NotFound));
    }

    #[test]
    fn accepts_robot_binary_names() {
        for name in ["robot", "robot.cmd", "robot.bat", "robot.exe"] {
            let path = format!("/definitely/not/{name}");
            let err = detect_robot(Some(&path)).unwrap_err();
            assert!(
                matches!(err, RobotError::NotFound),
                "{name} should be accepted by name check before existence"
            );
        }
    }

    #[test]
    fn path_probe_skips_missing_binary_without_which() {
        // ensure robot_exists_on_path returns false for a nonsense name (no panic / which).
        assert!(!robot_exists_on_path("ontocode-definitely-missing-robot-bin-xyz"));
    }
}
