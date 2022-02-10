use std::{process::{Command, Stdio}, fs, io};

pub fn repository_hash() -> Option<String> {
  crate_repository_hash().ok().or_else(|| git_repository_hash())
}

pub fn crate_repository_hash() -> io::Result<String> {
  let vcs_info = fs::read_to_string(".cargo_vcs_info.json")?;
  let value: serde_json::Value = serde_json::from_str(&vcs_info)?;
  let git = value.get("git").expect("failed to get 'git' property");
  let sha1 = git.get("sha1").expect("failed to get 'sha1' property");
  Ok(sha1.as_str().unwrap().into())
}

pub fn git_repository_hash() -> Option<String> {
  let mut cmd = Command::new("git");
  cmd.arg("rev-parse").arg("--short=20");
  let output = cmd.arg("HEAD").stderr(Stdio::inherit()).output().ok()?;
  if output.status.code() != Some(0) {
      None
  } else {
      // need to trim the string to remove newlines at the end.
      Some(String::from_utf8(output.stdout).unwrap().trim().to_string())
  }
}