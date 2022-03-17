use std::process::{Command, Stdio};
use std::{
    io::{self, Write},
    thread,
};

use letsdmmf::cli;
use letsdmmf::dmmf;
use letsdmmf::location;

fn main() -> io::Result<()> {
    let cmd = cli::get_cmd();

    let args = cli::get_args();
    let cli::Args {
        location,
        output,
        mode,
        scrolloff,
    } = args;

    let schema = match location::new(&location).get_schema() {
        Ok(schema) => schema,
        Err(message) => cli::error(cmd, message),
    };

    let dmmf = match dmmf::from(schema) {
        Ok(dmmf) => dmmf,
        Err(message) => cli::error(cmd, message),
    };

    match output {
        Some(path) => {
            let contents = serde_json::to_string_pretty(&dmmf).expect("Failed to stringify DMMF");

            std::fs::write(path, contents).expect("Failed to write output")
        }
        None => (),
    }

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
            .write_all(
                serde_json::to_string(&dmmf)
                    .expect("Failed to stringify DMMF")
                    .as_bytes(),
            )
            .expect("Failed to write to stdin");
    });

    child.wait()?;

    Ok(())
}
