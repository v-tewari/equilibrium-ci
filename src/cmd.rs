use clap::{value_parser, Arg, Command};

pub(crate) fn cli() -> Command {
    Command::new("equilibrium").version("0.1.0-predev.1").arg(
        Arg::new("port")
            .short('p')
            .long("port")
            .value_parser(value_parser!(u16))
            .help("port to which server will bind")
            .default_value("8080"),
    )
}
