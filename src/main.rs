use clap::{App, Arg, SubCommand};

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
        .about("Command-line Administration for Squad!")
        .arg(
            Arg::with_name("host")
                .short("h")
                .long("host")
                .value_name("HOST")
                .help("Rcon server to connect to in the form of ADDR:PORT")
                .required(true)
                .takes_value(true)
                .env("SQUAD_RCON_HOST"),
        )
        .arg(
            Arg::with_name("password")
                .short("p")
                .long("password")
                .value_name("PASSWORD")
                .help("Rcon password")
                .required(true)
                .takes_value(true)
                .env("SQUAD_RCON_PASS"),
        )
        .subcommand(SubCommand::with_name("monitor").about("Print incoming messages from server"))
        .subcommand(SubCommand::with_name("players").about("List the players on the server"))
        .subcommand(SubCommand::with_name("teams").about("List the teams on the server"))
        .subcommand(SubCommand::with_name("squads").about("List the squads on the server"))
        .subcommand(SubCommand::with_name("list_maps").about("List the maps on the server"))
        .subcommand(SubCommand::with_name("maps").about("Show the current and next map"))
        .subcommand(
            SubCommand::with_name("ban")
                .about("Ban player from server")
                .arg(
                    Arg::with_name("name")
                        .value_name("PLAYER")
                        .help("Player name or steamid")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("duration")
                        .value_name("DURATION")
                        .help("Examples: 1h (1 hour), 1m (1 month), 0 (indefinite)")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("reason")
                        .value_name("REASON")
                        .help("Reason for kicking the player")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("kick")
                .about("Kick player off server")
                .arg(
                    Arg::with_name("name")
                        .value_name("PLAYER")
                        .help("Player name or steamid")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("reason")
                        .value_name("REASON")
                        .help("Reason for kicking the player")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("set_next_map")
                .about("Set the next map to play on the server")
                .arg(
                    Arg::with_name("map")
                        .value_name("MAP")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("change_map")
                .about("Immediately end the current game and change the map")
                .arg(
                    Arg::with_name("map")
                        .value_name("MAP")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("broadcast")
                .about("Broadcast a message to the server")
                .arg(
                    Arg::with_name("message")
                        .value_name("MESSAGE")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("raw")
                .about("Send a raw command to the server")
                .arg(
                    Arg::with_name("command")
                        .value_name("COMMAND")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches();

    let host = matches.value_of("host").unwrap();
    let password = matches.value_of("password").unwrap();

    let mut squad_rcon = squad_rcon::SquadRcon::connect(host, password)?;

    if let Some(_) = matches.subcommand_matches("players") {
        for player in squad_rcon.players()? {
            println!(
                "{} - {} - {} - {}",
                player.name(),
                player.steam_id(),
                player.team(),
                player
                    .squad()
                    .map(|id| format!("{}", id))
                    .unwrap_or("N/A".to_string())
            );
        }
    } else if let Some(_) = matches.subcommand_matches("teams") {
        let (teams, _) = squad_rcon.squads()?;
        for team in teams {
            println!("{}: {}", team.id(), team.name());
        }
    } else if let Some(_) = matches.subcommand_matches("squads") {
        let (_, squads) = squad_rcon.squads()?;
        for squad in squads {
            println!(
                "{}: {} ({} players)",
                squad.id(),
                squad.name(),
                squad.size()
            );
        }
    } else if let Some(_) = matches.subcommand_matches("list_maps") {
        for map in squad_rcon.list_maps()? {
            println!("{}", map);
        }
    } else if let Some(_) = matches.subcommand_matches("maps") {
        let (current_map, next_map) = squad_rcon.maps()?;
        println!("Current map: {}", current_map);
        println!("Next map: {}", next_map);
    } else if let Some(matches) = matches.subcommand_matches("raw") {
        let command = matches.value_of("command").unwrap();
        println!("{}", squad_rcon.raw_command(command)?);
    } else if let Some(matches) = matches.subcommand_matches("broadcast") {
        let message = matches.value_of("message").unwrap();
        println!("{}", squad_rcon.broadcast(message)?);
    } else if let Some(matches) = matches.subcommand_matches("set_next_map") {
        let map = matches.value_of("map").unwrap();
        println!("{}", squad_rcon.set_next_map(map)?);
    } else if let Some(matches) = matches.subcommand_matches("change_map") {
        let map = matches.value_of("map").unwrap();
        println!("{}", squad_rcon.change_map(map)?);
    } else if let Some(matches) = matches.subcommand_matches("kick") {
        let name = matches.value_of("name").unwrap();
        let reason = matches.value_of("reason").unwrap();
        println!("{}", squad_rcon.kick(name, reason)?);
    } else if let Some(matches) = matches.subcommand_matches("ban") {
        let name = matches.value_of("name").unwrap();
        let duration = matches.value_of("duration").unwrap();
        let reason = matches.value_of("reason").unwrap();
        println!("{}", squad_rcon.ban(name, duration, reason)?);
    } else if let Some(_) = matches.subcommand_matches("monitor") {
        loop {
            let packet = squad_rcon.rcon_client_mut().recv_packet()?;
            println!("{}, {}, {}", packet.id(), packet.type_(), packet.body());
        }
    } else {
        println!("No command specified. Try --help");
    }

    Ok(())
}
