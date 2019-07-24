extern crate clap;

fn main() {
    let player_arg = clap::Arg::with_name("player")
        .short("p")
        .long("player")
        .takes_value(true)
        .multiple(true)
        .value_name("PLAYER_PATH");

    let clap_args = [player_arg];

    let args = clap::App::new("Filler_mk2")
        .version("0.1.0")
        .author("Random Guys")
        .about("About info")
        .args(&clap_args)
        .get_matches();

    match args.occurrences_of("player") {
        0 => panic!("No Players"),
        x if x == 1 || x == 2 => println!("No of players: {}", x),
        _ => panic!("Maximum of 2 players"),
    };

    let players: Vec<&str> = args
        .values_of("player")
        .expect("Error getting players")
        .collect();

    println!("{:?}", players);
}
