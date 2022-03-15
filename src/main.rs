use std::fs;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::thread;

use clap::IntoApp;
use clap::{ArgEnum, ErrorKind, Parser};

use letsdmmf::core;
use letsdmmf::location;
use letsdmmf::location::Location;
use letsdmmf::validate;

static ABOUT: &str = "Traverse DMMF of Prisma Schema, in your terminal";

static MODE: &str = concat!(
    r#"Initial viewing mode. "#,
    r#"In line mode (--mode line), opening and closing curly and square brackets are shown and all Object keys are quoted. "#,
    r#"In data mode (--mode data; the default), closing braces, commas, and quotes around Object keys are elided."#,
    r#"The active mode can be toggled by pressing 'm'"#
);

static SCROLLOFF: &str = concat!(
    r#"Number of lines to maintain as padding between the currently focused row and the top or bottom of the screen."#,
    r#"Setting this to a large value will keep the focused in the middle of the screen (except at the start or end of a file)"#
);

#[derive(Parser)]
#[clap(author, version, about = ABOUT, long_about = None)]
struct Args {
    path: String,

    #[clap(short, long, default_value_t = Mode::Data, help = MODE, arg_enum)]
    mode: Mode,
    #[clap(
        long,
        parse(try_from_str),
        default_value_t = 3,
        help = SCROLLOFF
    )]
    scrolloff: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, ArgEnum)]
enum Mode {
    Data,
    Line,
}

fn get_schema(location: String) -> Result<String, String> {
    let location_type = location::new(&location);

    match location_type {
        Location::Path(path) => match validate::path(&path) {
            Ok(()) => {
                let schema = fs::read_to_string(path).expect("Failed to read schema from path");

                Ok(schema)
            }
            Err(message) => Err(message),
        },
        Location::Url(url) => match validate::url(&url) {
            Ok(url) => {
                let schema = reqwest::blocking::get(url)
                    .expect("Failed to get response")
                    .text()
                    .expect("Failed to convert response to text");

                Ok(schema)
            }
            Err(message) => Err(message),
        },
    }
}

fn main() -> io::Result<()> {
    let mut cmd = Args::command();

    let args = Args::parse();
    let Args {
        path,
        mode,
        scrolloff,
    } = args;

    let schema = match get_schema(path) {
        Ok(schema) => schema,
        Err(message) => {
            cmd.error(ErrorKind::ValueValidation, message).exit();
        }
    };

    let dmmf = match core::dmmf(schema) {
        Ok(dmmf) => dmmf,
        Err(message) => {
            cmd.error(ErrorKind::ValueValidation, message).exit();
        }
    };

    let child_result = Command::new("jless")
        .stdin(Stdio::piped())
        .args(["--scrolloff", &(scrolloff.to_string())])
        .args([
            "--mode",
            match mode {
                Mode::Data => "data",
                Mode::Line => "line",
            },
        ])
        .spawn();

    let mut child = match child_result {
        Ok(child) => child,
        Err(_e) => {
            cmd.error(ErrorKind::Io, "Failed to run \"jless\" properly")
                .exit();
        }
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
