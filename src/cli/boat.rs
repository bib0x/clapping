use clap::*;

pub fn build_boat_commands() -> Command<'static> {
    Command::new("boat")
        .subcommand(
            Command::new("ls")
                .about("List boats from a boat")
                .arg(
                    Arg::new("Boat's name").long("name").short('n')
                )
        )
}

