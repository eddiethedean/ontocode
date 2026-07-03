use crate::error::{Result, RobotError};
use std::path::Path;
use std::process::Command;

pub struct RobotOutput {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

pub fn detect_robot(explicit_path: Option<&str>) -> Result<String> {
    if let Some(path) = explicit_path.filter(|p| !p.is_empty()) {
        if Path::new(path).exists() {
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
    let robot = detect_robot(robot_path)?;
    let output = Command::new(&robot).args(args).output()?;
    let exit_code = output.status.code().unwrap_or(1);
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    Ok(RobotOutput { exit_code, stdout, stderr })
}

pub fn robot_validate(robot_path: Option<&str>, ontology_path: &Path) -> Result<RobotOutput> {
    run_robot(robot_path, &["validate".to_string(), ontology_path.display().to_string()])
}

pub fn robot_merge(
    robot_path: Option<&str>,
    inputs: &[String],
    output: &Path,
) -> Result<RobotOutput> {
    let mut args = vec!["merge".to_string(), "--output".to_string(), output.display().to_string()];
    for input in inputs {
        args.push("--input".to_string());
        args.push(input.clone());
    }
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
            ontology_path.display().to_string(),
            "--report".to_string(),
            report_path.display().to_string(),
        ],
    )
}
