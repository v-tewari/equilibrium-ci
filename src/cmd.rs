use clap::{value_parser, Arg, Command};

pub(crate) fn cli() -> Command {
    Command::new("equilibrium")
        .version("0.1.0-predev.1")
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_parser(value_parser!(u16))
                .help("port to which server will bind")
                .default_value("8080"),
        )
        .arg(
            Arg::new("conn_str")
                .long("db-url")
                .help("database connection string")
                .required(true),
        )
        .arg(
            Arg::new("max_connections")
                .short('m')
                .long("max-connection")
                .value_parser(value_parser!(u32))
                .help("maximum number of connections")
                .default_value("5"),
        )
}
