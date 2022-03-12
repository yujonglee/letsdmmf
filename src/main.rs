use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::thread;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let schema_path = args
        .iter()
        .find(|&arg| (*arg).ends_with(".prisma"))
        .expect("Schema path is required");

    let schema = fs::read_to_string(schema_path).expect("Failed to read schema from path");

    let (_cfg, dml) = datamodel::parse_schema(&schema).expect("Failed to parse schema");
    let dmmf = dmmf::render_to_dmmf(&dml);

    let mut child = Command::new("jless")
        .stdin(Stdio::piped())
        .args(["-m", "line"])
        .spawn()
        .expect("Failed to run `jless`");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");

    thread::spawn(move || {
        stdin
            .write_all((&dmmf).as_bytes())
            .expect("Failed to write to stdin");
    });

    child.wait()?;

    Ok(())
}
