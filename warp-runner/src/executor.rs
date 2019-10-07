use std::env;
use std::io;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

pub fn execute(target: &Path) -> io::Result<i32> {
    trace!("target={:?}", target);

    let mut args: Vec<String> = env::args().skip(1).collect();
    let self_path = env::current_exe().ok().unwrap();
    let file_name = self_path.file_name().unwrap().to_str().unwrap();
    let file_name_index = self_path.to_str().unwrap().rfind(file_name).unwrap();
    let working_directory: String = self_path.to_str().unwrap().chars().take(file_name_index).collect();
    args.push("--workingDirectory".to_string());
    args.push(working_directory);
    trace!("args={:?}", args);

    do_execute(target, &args)
}

#[cfg(target_family = "unix")]
fn do_execute(target: &Path, args: &[String]) -> io::Result<i32> {
    Ok(Command::new(target)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?
        .code().unwrap_or(1))
}

#[cfg(target_family = "windows")]
fn is_script(target: &Path) -> bool {
    const SCRIPT_EXTENSIONS: &[&str] = &["bat", "cmd"];
    SCRIPT_EXTENSIONS.contains(
        &target.extension()
            .unwrap_or_default()
            .to_string_lossy()
            .to_lowercase().as_str())
}

#[cfg(target_family = "windows")]
fn do_execute(target: &Path, args: &[String]) -> io::Result<i32> {
    let target_str = target.as_os_str().to_str().unwrap();

    if is_script(target) {
        let mut cmd_args = Vec::with_capacity(args.len() + 2);
        cmd_args.push("/c".to_string());
        cmd_args.push(target_str.to_string());
        cmd_args.extend_from_slice(&args);

        Ok(Command::new("cmd")
            .args(cmd_args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?
            .wait()?
            .code().unwrap_or(1))
    } else {
        Ok(Command::new(target)
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?
            .wait()?
            .code().unwrap_or(1))
    }
}