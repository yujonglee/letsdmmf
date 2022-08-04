use std::{
    io::{self, Write},
    process::{Command, Stdio},
    str::FromStr,
    thread,
};

use inquire::{Confirm, Select};

use letsdmmf::example::Relation;
use letsdmmf::location;
use letsdmmf::{cli, prisma};

fn main() -> io::Result<()> {
    let cmd = cli::get_cmd();

    let args = cli::get_args();
    let cli::Args {
        location,
        output,
        mode,
        scrolloff,
    } = args;

    let location = match location.as_str() {
        "example" => {
            let options = vec![
                "1-1",
                "1-1-self",
                "1-1-multi-field",
                "1-n",
                "1-n-self",
                "1-n-multi-field",
                "m-n-explicit",
                "m-n-implicit",
                "m-n-self",
                "m-n-self-explicit",
            ];

            let selected_schema = match Select::new("Select the schema you want", options).prompt()
            {
                Ok(option) => Relation::from_str(option).unwrap(),
                Err(_e) => cli::error(cmd, String::from("Failed to select one of examples")),
            };

            let is_doc_want = Confirm::new("Do you want to read Prisma's documentation about it?")
                .with_default(false)
                .with_help_message("This will open your web broswer")
                .prompt();

            match is_doc_want {
                Ok(true) => {
                    let doc_url = selected_schema.get_doc_url();

                    webbrowser::open(&doc_url).expect("Failed to open web broswer");
                }
                Ok(false) => (),
                Err(_) => cli::error(cmd, String::from("Failed to select one of options")),
            }

            location::Location::Example(selected_schema)
        }
        _ => location::new(location),
    };

    let schema = match location.get_schema() {
        Ok(schema) => schema,
        Err(message) => cli::error(cmd, message),
    };

    let dmmf = prisma::dmmf_from_schema(&schema);

    match output {
        Some(path) => {
            if std::path::Path::new(&path).is_dir() {
                cli::error(
                    cmd,
                    format!(
                        "Expect file path, got directory instead. Did you mean \"{}/dmmf.json\"?",
                        path
                    ),
                )
            }

            let contents = serde_json::to_string_pretty(&dmmf).expect("Failed to stringify DMMF");

            std::fs::write(path, contents).expect("Failed to write output")
        }
        None => {
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
        }
    }

    Ok(())
}
