use anyhow::{bail, Context, Result};
use std::process::Command;

fn main() -> Result<()> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .context("Failed to execute 'git' command")?;

    if output.status.success() {
        let path_str = String::from_utf8_lossy(&output.stdout);
        print!("{}", path_str.trim());
        Ok(())
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        bail!(
            "git command failed (exit code: {}):\n{}",
            output.status.code().unwrap_or(-1),
            error_msg.trim()
        );
    }
}
