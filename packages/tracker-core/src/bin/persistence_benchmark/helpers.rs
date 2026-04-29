use std::process::Command;

#[must_use]
pub fn git_revision() -> String {
    match Command::new("git").args(["rev-parse", "HEAD"]).output() {
        Ok(output) if output.status.success() => {
            let revision = String::from_utf8_lossy(&output.stdout);
            revision.trim().to_string()
        }
        _ => "unknown".to_string(),
    }
}
