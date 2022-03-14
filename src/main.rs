use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;

use clap::IntoApp;
use clap::{ArgEnum, ErrorKind, Parser};

static ABOUT: &str = concat!(r#"Traverse DMMF of your Prisma Schema, in your command"#);

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

fn validate_path(path: &str) {
    let is_exist = Path::new(&path).exists();
    let mut cmd = Args::command();

    if is_exist {
        if !path.ends_with(".prisma") {
            let file_name = Path::new(&path).file_name().unwrap();

            cmd.error(
                ErrorKind::ValueValidation,
                format!(
                    "Invalid File Extension, \"something.prisma\" expected, found {:?}",
                    file_name
                ),
            )
            .exit();
        }
    } else {
        cmd.error(
            ErrorKind::ValueValidation,
            format!("No such file: \"{}\"", path),
        )
        .exit();
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

    validate_path(&path);

    let schema = fs::read_to_string(path).expect("Failed to read schema from path");

    let (_cfg, dml) = datamodel::parse_schema(&schema).expect("Failed to parse schema");
    let dmmf = dmmf::render_to_dmmf(&dml);

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
            cmd.error(ErrorKind::Io, "Failed to run \"jelss\" properly")
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
