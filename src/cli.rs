use clap::{ArgEnum, Command, ErrorKind, IntoApp, Parser};

#[derive(Parser)]
#[clap(author, version, about = ABOUT, long_about = None)]
pub struct Args {
    #[clap(help = LOCATION)]
    pub location: String,

    #[clap(short, long, help = OUTPUT)]
    pub output: Option<String>,

    #[clap(short, long, default_value_t = Mode::Data, help = MODE, arg_enum)]
    pub mode: Mode,

    #[clap(
        short,
        long,
        parse(try_from_str),
        default_value_t = 3,
        help = SCROLLOFF
    )]
    pub scrolloff: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, ArgEnum)]
pub enum Mode {
    Data,
    Line,
}

static ABOUT: &str = "Traverse DMMF of Prisma Schema, in your terminal";

static LOCATION: &str = "Location of Prisma schema. Can be file path or url.\nProvide \"example\" if you want to see examples";

static OUTPUT: &str = "Output file path. If specified, the viewer will not open";

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

pub fn get_cmd() -> Command<'static> {
    Args::command()
}

pub fn get_args() -> Args {
    Args::parse()
}

pub fn error(mut cmd: Command, message: String) -> ! {
    cmd.error(ErrorKind::ValueValidation, message).exit();
}
