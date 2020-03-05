use crate::rcon::RconClient;
use crate::Error;
use lazy_static::lazy_static;
use log::{error, info};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::net::ToSocketAddrs;

// pub const SERVERDATA_CHAT: i32 = 1;

lazy_static! {
    static ref PLAYER_REGEX: Regex =
        Regex::new(r"ID: (\d*) \| SteamID: (\d*) \| Name: (.*) \| Team ID: (\d) \| Squad ID: (.*)")
            .unwrap();
    static ref SQUAD_REGEX: Regex =
        Regex::new(r"ID: (\d*) \| Name: (.*) \| Size: (\d) \| Locked: (.*)").unwrap();
    static ref TEAM_REGEX: Regex = Regex::new(r"Team ID: (\d*) \((.*)\)").unwrap();
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Player {
    id: usize,
    steam_id: String,
    name: String,
    team: usize,
    squad: Option<usize>,
}

impl Player {
    pub fn new(
        id: usize,
        steam_id: String,
        name: String,
        team: usize,
        squad: Option<usize>,
    ) -> Player {
        Player {
            id: id,
            steam_id: steam_id,
            name: name,
            team: team,
            squad: squad,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
    pub fn steam_id(&self) -> &str {
        &self.steam_id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn team(&self) -> usize {
        self.team.clone()
    }
    pub fn squad(&self) -> Option<usize> {
        self.squad
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Squad {
    id: usize,
    name: String,
    size: usize,
    team: usize,
    locked: bool,
}

impl Squad {
    pub fn new(id: usize, name: String, size: usize, team: usize, locked: bool) -> Squad {
        Squad {
            id: id,
            name: name,
            size: size,
            team: team,
            locked: locked,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn team(&self) -> usize {
        self.team
    }
    pub fn locked(&self) -> bool {
        self.locked
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Team {
    id: usize,
    name: String,
}

impl Team {
    pub fn new(id: usize, name: String) -> Team {
        Team { id: id, name: name }
    }

    pub fn id(&self) -> usize {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

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

    pub fn rcon_client_mut(&mut self) -> &mut RconClient {
        &mut self.rcon_client
    }

    pub fn raw_command<S: Into<String>>(&mut self, command: S) -> Result<String, Error> {
        self.rcon_client.exec_command(command)
    }

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

            let team_id = captures.get(4).unwrap().as_str().parse().unwrap_or(0);
            let squad_id = captures.get(5).unwrap().as_str().parse().ok();

            let player = Player::new(
                captures.get(1).unwrap().as_str().parse()?,
                captures.get(2).unwrap().as_str().to_string(),
                captures.get(3).unwrap().as_str().to_string(),
                team_id,
                squad_id,
            );

            players.push(player);
        }

        Ok(players)
    }

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
                let id = captures.get(1).unwrap().as_str().parse().unwrap_or(0);
                let name = captures.get(2).unwrap().as_str().to_string();

                current_team = id;
                let team = Team::new(id, name);
                teams.push(team);
            } else if let Some(captures) = SQUAD_REGEX.captures(&line) {
                let id = captures.get(1).unwrap().as_str().parse().unwrap_or(0);
                let name = captures.get(2).unwrap().as_str().to_string();
                let size = captures.get(3).unwrap().as_str().parse().unwrap_or(0);
                let locked = captures.get(4).unwrap().as_str() == "True";
                let squad = Squad::new(id, name, size, current_team, locked);

                squads.push(squad);
            } else {
                error!("Could not parse squad line: {}", line);
            }
        }

        Ok((teams, squads))
    }

    pub fn maps(&mut self) -> Result<Vec<String>, Error> {
        let maps_string = self.raw_command("ListMaps")?;

        Ok(maps_string
            .split("\n")
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>())
    }

    pub fn change_map<S: AsRef<str>>(&mut self, map: S) -> Result<(), Error> {
        info!(
            "{}",
            self.raw_command(format!("AdminChangeMap {}", map.as_ref()))?
        );
        Ok(())
    }

    pub fn set_next_map<S: AsRef<str>>(&mut self, map: S) -> Result<(), Error> {
        info!(
            "{}",
            self.raw_command(format!("AdminSetNextMap {}", map.as_ref()))?
        );
        Ok(())
    }

    /// Name can be player name or steam64id
    pub fn force_team_change<S: AsRef<str>>(&mut self, name: S) -> Result<(), Error> {
        info!(
            "{}",
            self.raw_command(format!("AdminForceTeamChange {}", name.as_ref()))?
        );
        Ok(())
    }

    /// Name can be player name or steam64id
    pub fn demote_commander<S: AsRef<str>>(&mut self, name: S) -> Result<(), Error> {
        info!(
            "{}",
            self.raw_command(format!("AdminDemoteCommander {}", name.as_ref()))?
        );
        Ok(())
    }

    /// Name can be player name or steam64id
    pub fn disband_squad<S: AsRef<str>>(
        &mut self,
        team_id: usize,
        squad_id: usize,
    ) -> Result<(), Error> {
        info!(
            "{}",
            self.raw_command(format!("AdminDisbandSquad {} {}", team_id, squad_id))?
        );
        Ok(())
    }

    /// Broadcast an administrative message to the server
    pub fn broadcast<M>(&mut self, message: M) -> Result<(), Error>
    where
        M: AsRef<str>,
    {
        info!(
            "{}",
            self.raw_command(format!("AdminBroadcast {}", message.as_ref()))?
        );

        Ok(())
    }

    /// Broadcast an administrative message to the server
    pub fn chat_to_admin<M>(&mut self, message: M) -> Result<(), Error>
    where
        M: AsRef<str>,
    {
        info!(
            "{}",
            self.raw_command(format!("ChatToAdmin {}", message.as_ref()))?
        );

        Ok(())
    }

    /// Warn a player by name or steamid
    /// `name` can be the player's name, or their steam64id
    pub fn warn<N, R>(&mut self, name: N, reason: R) -> Result<(), Error>
    where
        N: AsRef<str>,
        R: AsRef<str>,
    {
        info!(
            "{}",
            self.raw_command(format!(
                "AdminWarn \"{}\" {}",
                name.as_ref(),
                reason.as_ref()
            ))?
        );

        Ok(())
    }

    /// Kick a player by name or steamid
    /// `name` can be the player's name, or their steam64id
    pub fn kick<N, R>(&mut self, name: N, reason: R) -> Result<(), Error>
    where
        N: AsRef<str>,
        R: AsRef<str>,
    {
        info!(
            "{}",
            self.raw_command(format!(
                "AdminKick \"{}\" {}",
                name.as_ref(),
                reason.as_ref()
            ))?
        );

        Ok(())
    }

    /// Ban a user for a diven amount of time
    /// Length can be:
    /// * 1d = one day
    /// * 1m = one month
    /// * 0 = permanent ban
    pub fn ban<N, L, R>(&mut self, name: N, length: L, reason: R) -> Result<(), Error>
    where
        N: AsRef<str>,
        L: AsRef<str>,
        R: AsRef<str>,
    {
        info!(
            "{}",
            self.raw_command(format!(
                "AdminBan \"{}\" \"{}\" {}",
                name.as_ref(),
                length.as_ref(),
                reason.as_ref()
            ))?
        );

        Ok(())
    }
}
