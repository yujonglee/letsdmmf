use std::process::{Command, Stdio};
use std::{
    io::{self, Write},
    thread,
};

use letsdmmf::cli;
use letsdmmf::core;

fn main() -> io::Result<()> {
    let cmd = cli::get_cmd();

    let args = cli::get_args();
    let cli::Args {
        path,
        mode,
        scrolloff,
    } = args;

    let schema = match core::get_schema(path) {
        Ok(schema) => schema,
        Err(message) => cli::error(cmd, message),
    };

    let dmmf = match core::get_dmmf(schema) {
        Ok(dmmf) => dmmf,
        Err(message) => cli::error(cmd, message),
    };

    let child_result = Command::new("jless")
        .stdin(Stdio::piped())
        .args(["--scrolloff", &(scrolloff.to_string())])
        .args([
            "--mode",
            match mode {
                cli::Mode::Data => "data",
                cli::Mode::Line => "line",
            },
        ])
        .spawn();

    let mut child = match child_result {
        Ok(child) => child,
        Err(_e) => cli::error(cmd, String::from("Failed to run \"jless\" properly")),
    };

    let mut stdin = child.stdin.take().expect("Failed to open stdin");

    thread::spawn(move || {
        stdin
            .write_all((&dmmf).as_bytes())
            .expect("Failed to write to stdin");
    });

    child.wait()?;

    Ok(())
}
