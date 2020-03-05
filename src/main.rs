use clap::{App, Arg};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use simplelog::{CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode};
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Mixed,
    )
    .unwrap()])
    .unwrap();

    let matches = App::new("Squad Rcon")
        .about("Administer Squad Servers with Rust!")
        .arg(
            Arg::with_name("host")
                .short("h")
                .long("host")
                .value_name("HOST")
                .help("Rcon server to connect to in the form of ADDR:PORT")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("password")
                .short("p")
                .long("password")
                .value_name("PASSWORD")
                .help("Rcon password")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let host = matches.value_of("host").unwrap();
    let password = matches.value_of("password").unwrap();

    let mut squad_rcon =
        squad_rcon::SquadRcon::connect(host, password)?;

    println!("Players");
    for player in squad_rcon.players()? {
        println!(
            "{} - {} - {} - {}",
            player.name(),
            player.steam_id(),
            player.team(),
            player.squad().map(|id| format!("{}", id)).unwrap_or("N/A".to_string())
        );
    }

    let (teams, squads) = squad_rcon.squads()?;

    println!("Teams");
    for team in teams {
        println!("{}: {}", team.id(), team.name());
    }

    println!("Squads");
    for squad in squads {
        println!(
            "{}: {} ({} players)",
            squad.id(),
            squad.name(),
            squad.size()
        );
    }

    Ok(())
}
