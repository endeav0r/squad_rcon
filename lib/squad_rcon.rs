use crate::rcon::RconClient;
use crate::{Error, Player, Squad, Team};
use lazy_static::lazy_static;
use regex::Regex;
use std::net::ToSocketAddrs;

pub const SERVERDATA_CHAT: i32 = 1;

lazy_static! {
    static ref PLAYER_REGEX: Regex =
        Regex::new(r"ID: (\d*) \| SteamID: (\d*) \| Name: (.*) \| Team ID: (\d) \| Squad ID: (.*)")
            .expect("PLAYER_REGEX");
    static ref SQUAD_REGEX: Regex =
        Regex::new(r"ID: (\d*) \| Name: (.*) \| Size: (\d) \| Locked: (.*)").expect("SQUAD_REGEX");
    static ref TEAM_REGEX: Regex = Regex::new(r"Team ID: (\d*) \((.*)\)").expect("TEAM_REGEX");
    static ref MAPS_REGEX: Regex =
        Regex::new(r"Current map is (.*), Next map is (.*)").expect("MAPS_REGEX");
}

/// A squad-specific rcon connection
pub struct SquadRcon {
    rcon_client: RconClient,
}

impl SquadRcon {
    pub fn connect<A: ToSocketAddrs, S: Into<String>>(
        addr: A,
        password: S,
    ) -> Result<SquadRcon, Error> {
        Ok(SquadRcon {
            rcon_client: RconClient::connect(addr, password)?,
        })
    }

    /// Get a mutable reference to the underlying rcon connection
    pub fn rcon_client_mut(&mut self) -> &mut RconClient {
        &mut self.rcon_client
    }

    /// Execute a raw rcon command, and return the result.
    ///
    /// This is a convenience wrapper around `RconClient::exec_command`.
    pub fn raw_command<S: Into<String>>(&mut self, command: S) -> Result<String, Error> {
        self.rcon_client.exec_command(command)
    }

    /// Return all of the players on the squad server
    pub fn players(&mut self) -> Result<Vec<Player>, Error> {
        let players_string = self.raw_command("ListPlayers")?;

        let mut lines = players_string
            .split("\n")
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        lines.remove(0);

        let mut players = Vec::new();

        for line in lines {
            if line.contains("Recently Disconnected Players") {
                break;
            }

            let captures = PLAYER_REGEX
                .captures(&line)
                .ok_or(Error::SquadParsingError)?;

            let team_id = captures
                .get(4)
                .expect("players get 4")
                .as_str()
                .parse()
                .unwrap_or(0);
            let squad_id = captures
                .get(5)
                .expect("players get 5")
                .as_str()
                .parse()
                .ok();

            let player = Player::new(
                captures.get(1).expect("players get 1").as_str().parse()?,
                captures.get(2).expect("players get 2").as_str().to_string(),
                captures.get(3).expect("players get 3").as_str().to_string(),
                team_id,
                squad_id,
            );

            players.push(player);
        }

        Ok(players)
    }

    /// Return all of the teams and squads on the server.
    ///
    /// These are returned in one call by the server as per the squad rcon
    /// protocol. This is one underlying rcon command.
    pub fn squads(&mut self) -> Result<(Vec<Team>, Vec<Squad>), Error> {
        let squads_string = self.raw_command("ListSquads")?;

        let mut lines = squads_string
            .split("\n")
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        lines.remove(0);

        let mut teams = Vec::new();
        let mut squads = Vec::new();

        let mut current_team = 0;

        for line in lines {
            if let Some(captures) = TEAM_REGEX.captures(&line) {
                let id = captures
                    .get(1)
                    .expect("squads team get 1")
                    .as_str()
                    .parse()
                    .unwrap_or(0);
                let name = captures
                    .get(2)
                    .expect("squads team get 2")
                    .as_str()
                    .to_string();

                current_team = id;
                let team = Team::new(id, name);
                teams.push(team);
            } else if let Some(captures) = SQUAD_REGEX.captures(&line) {
                let id = captures
                    .get(1)
                    .expect("squads get 1")
                    .as_str()
                    .parse()
                    .unwrap_or(0);
                let name = captures.get(2).expect("squads get 2").as_str().to_string();
                let size = captures
                    .get(3)
                    .expect("squads get 3")
                    .as_str()
                    .parse()
                    .unwrap_or(0);
                let locked = captures.get(4).expect("squads get 4").as_str() == "True";
                let squad = Squad::new(id, name, size, current_team, locked);

                squads.push(squad);
            } else {
                return Err(Error::SquadParsingError);
            }
        }

        Ok((teams, squads))
    }

    /// Return a list of all the maps supported by the server
    pub fn list_maps(&mut self) -> Result<Vec<String>, Error> {
        let maps_string = self.raw_command("ListMaps")?;

        Ok(maps_string
            .split("\n")
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>())
    }

    /// End the current match
    pub fn end_match(&mut self) -> Result<String, Error> {
        self.raw_command("AdminEndMatch")
    }

    /// Change the map currently running on the squad server.
    pub fn change_map<S: AsRef<str>>(&mut self, map: S) -> Result<String, Error> {
        self.raw_command(format!("AdminChangeMap {}", map.as_ref()))
    }

    /// Set the map which will run on the squad server when the current game is
    /// finished.
    pub fn set_next_map<S: AsRef<str>>(&mut self, map: S) -> Result<String, Error> {
        self.raw_command(format!("AdminSetNextMap {}", map.as_ref()))
    }

    /// Force a player onto the other team.
    ///
    /// `name` can be player name or steam64id.
    pub fn force_team_change<S: AsRef<str>>(&mut self, name: S) -> Result<String, Error> {
        self.raw_command(format!("AdminForceTeamChange {}", name.as_ref()))
    }

    /// Demote the commander.
    ///
    /// `name` can be player name or steam64id.
    pub fn demote_commander<S: AsRef<str>>(&mut self, name: S) -> Result<String, Error> {
        self.raw_command(format!("AdminDemoteCommander {}", name.as_ref()))
    }

    /// Disband a squad.
    pub fn disband_squad<S: AsRef<str>>(
        &mut self,
        team_id: usize,
        squad_id: usize,
    ) -> Result<String, Error> {
        self.raw_command(format!("AdminDisbandSquad {} {}", team_id, squad_id))
    }

    /// Broadcast an administrative message to the server.
    pub fn broadcast<M>(&mut self, message: M) -> Result<String, Error>
    where
        M: AsRef<str>,
    {
        self.raw_command(format!("AdminBroadcast {}", message.as_ref()))
    }

    /// Send a message to admin chat, which only admins can see.
    pub fn chat_to_admin<M>(&mut self, message: M) -> Result<String, Error>
    where
        M: AsRef<str>,
    {
        self.raw_command(format!("ChatToAdmin {}", message.as_ref()))
    }

    /// Warn a player by name or steamid
    ///
    /// `name` can be the player's name, or their steam64id
    pub fn warn<N, R>(&mut self, name: N, reason: R) -> Result<String, Error>
    where
        N: AsRef<str>,
        R: AsRef<str>,
    {
        self.raw_command(format!(
            "AdminWarn \"{}\" {}",
            name.as_ref(),
            reason.as_ref()
        ))
    }

    /// Kick a player by name or steamid
    ///
    /// `name` can be the player's name, or their steam64id
    pub fn kick<N, R>(&mut self, name: N, reason: R) -> Result<String, Error>
    where
        N: AsRef<str>,
        R: AsRef<str>,
    {
        self.raw_command(format!(
            "AdminKick \"{}\" {}",
            name.as_ref(),
            reason.as_ref()
        ))
    }

    /// Ban a user for a diven amount of time
    /// Length can be:
    /// * 1d = one day
    /// * 1m = one month
    /// * 0 = permanent ban
    ///
    /// `name` can be the player's name, or their steam64id
    pub fn ban<N, L, R>(&mut self, name: N, length: L, reason: R) -> Result<String, Error>
    where
        N: AsRef<str>,
        L: AsRef<str>,
        R: AsRef<str>,
    {
        self.raw_command(format!(
            "AdminBan \"{}\" \"{}\" {}",
            name.as_ref(),
            length.as_ref(),
            reason.as_ref()
        ))
    }

    /// Get the current map, and the next map
    ///
    /// Result is (current_map, next_map)
    pub fn maps(&mut self) -> Result<(String, String), Error> {
        let maps_string = self.raw_command("ShowNextMap")?;

        let captures = MAPS_REGEX
            .captures(&maps_string)
            .ok_or(Error::SquadParsingError)?;

        Ok((
            captures.get(1).expect("maps 1").as_str().to_string(),
            captures.get(2).expect("maps 2").as_str().to_string(),
        ))
    }
}
