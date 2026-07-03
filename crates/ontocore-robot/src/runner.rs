use crate::error::{Result, RobotError};
use std::path::{Component, Path};
use std::process::Command;

pub struct RobotOutput {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

const ALLOWED_SUBCOMMANDS: &[&str] =
    &["validate-profile", "validate", "merge", "report", "reason", "query"];

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
    let which = Command::new("which").arg("robot").output();
    if let Ok(output) = which {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return Ok(path);
            }
        }
    }
    Err(RobotError::NotFound)
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
