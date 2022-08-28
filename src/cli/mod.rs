use clap::*;

pub mod boat;
pub mod room;


pub fn build_cli() -> Command<'static> {
    
    Command::new("clapping")
        .about("A Crab getting to know boats when clapping")
        .subcommand(room::build_room_commands())
        .subcommand(boat::build_boat_commands())
}
