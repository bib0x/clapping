use clap::*;

pub fn build_room_commands() -> Command<'static> {
    Command::new("room")
        .subcommand(
            Command::new("ls")
                .about("List rooms from a boat")
                .arg(
                    Arg::new("name").long("name").short('n').help("Room's name")
                )
        )
}
