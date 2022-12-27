use std::path::Path;
use std::process::Command;

pub fn is_within_git_repo(path: &Path) -> bool {
    let output = Command::new("git")
                .arg("rev-parse")
                .arg("--git-dir")
                .output()
                .expect("failed to execute process");
    if output.status.success() {
        true
    } else {
        false
    }
}

pub fn is_git_repo_head(path: &Path) -> bool {
    let git_dir = path.join(".git");
    git_dir.exists() && git_dir.is_dir()
}

pub fn get_git_files(path: &Path) -> Vec<String> {
    let output = Command::new("git")
                .arg("ls-files")
                .arg(path.to_string_lossy().to_string())
                .output()
                .expect("failed to execute process");

    let git_files = String::from_utf8(output.stdout).expect("invalid utf8");
    git_files.lines().map(|s| s.to_string()).collect()
}