
/// Constantly monitors a squad server for new messages. Sends keepalives, and
/// attempts to reconnect if the server goes down.
pub struct Monitor {
    squad_rcon: SquadRcon
}

impl Monitor {
    pub fn connect<A: ToSocketAddrs, S: Into<String>>(
        addr: A,
        password: S,
    ) -> Result<SquadRcon, Error> {
        Ok(Monitor {
            squad_rcon: SquadRcon::connect(addr, password)?
        })
    }
}