mod cli;

fn main() {
    let cli = cli::build_cli().get_matches();

    println!("{:?}", cli);
}
