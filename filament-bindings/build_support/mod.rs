mod target;
mod download;
use std::{process::Command, io::ErrorKind};

pub use target::*;
pub use download::*;

#[cfg(not(target_os = "windows"))]
pub fn path_regex_escape(path: impl AsRef<str>) -> String {
    regex::escape(path.as_ref())
}

#[cfg(target_os = "windows")]
pub fn path_regex_escape(path: impl AsRef<str>) -> String {
    let re = regex::Regex::new(r"[/\\]+").unwrap();
    re.split(path.as_ref())
        .map(|segment| regex::escape(segment))
        .collect::<Vec<String>>()
        .join(r"[/\\]+")
}


pub fn run_command(cmd: &mut Command, program: &str) {
  println!(
      "current_dir: {:?}\nrunning: {:?}",
      cmd.get_current_dir()
          .map(|p| p.display().to_string())
          .unwrap_or("".to_string()),
      cmd
  );
  let status = match cmd.status() {
      Ok(status) => status,
      Err(ref e) if e.kind() == ErrorKind::NotFound => {
          panic!(
              "{}",
              &format!(
                  "failed to execute command: {}\nis `{}` not installed?",
                  e, program
              )
          );
      }
      Err(e) => panic!("{}", &format!("failed to execute command: {:?}", e)),
  };
  if !status.success() {
      panic!(
          "{}",
          &format!("command did not execute successfully, got: {}", status)
      );
  }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn static_lib_filename(lib_name: &str) -> String {
    format!("lib{}.a", lib_name)
}

#[cfg(target_os = "windows")]
pub fn static_lib_filename(lib_name: &str) -> String {
    format!("{}.lib", lib_name)
}
