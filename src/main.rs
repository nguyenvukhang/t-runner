use std::os::unix::process::CommandExt;
use std::process::Command;
use std::{env, fs, io};

macro_rules! forward {
    ($cmd:expr) => {
        Command::new($cmd).args(env::args_os().skip(1))
    };
    ($cmd:expr, $($args:expr),+) => {
        Command::new($cmd).args([$($args),*]).args(env::args_os().skip(1))
    };
}

macro_rules! rule {
    ($filepath:expr => $($args:expr),+) => {
        if let Ok(true) = fs::exists($filepath) {
            // println!("Found [\x1b[32m{}\x1b[m]!", $filepath);
            return Err(forward!($($args),+).exec())
        }
    }
}

fn main() -> io::Result<()> {
    let mut cwd = env::current_dir()?;

    while cwd.parent().map_or(false, |v| !v.as_os_str().is_empty()) {
        rule!("Makefile" => "make");
        rule!("Cargo.toml" => "cargo", "run");
        rule!("run" => "bash", "run");
        rule!("build.sh" => "bash", "build.sh");
        rule!("run.py" => "python3", "run.py");
        cwd.pop();
        let Ok(_) = env::set_current_dir(&cwd) else { return Ok(()) };
    }

    Ok(())
}
