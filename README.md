# Squad Rcon

This is a rust library and command-line utilityfor interacting with, and
administering, [Squad](https://joinsquad.com) servers.

To install:

```
$ git clone https://github.com/endeav0r/squad_rust
$ cd squad_rust
$ cargo install --path .
$ squad-rcon -h <addr:port> -p <rcon_password> players
```

You can set the environment variables `SQUAD_RCON_HOST` and `SQUAD_RCON_PASS`,
preventing you from having to pass these arguments every time you run the
utility.

```
Squad Rcon 
Command-line Administration for Squad!

USAGE:
    squad-rcon --host <HOST> --password <PASSWORD> [SUBCOMMAND]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -h, --host <HOST>            Rcon server to connect to in the form of ADDR:PORT
    -p, --password <PASSWORD>    Rcon password

SUBCOMMANDS:
    ban             Ban player from server
    broadcast       Broadcast a message to the server
    change_map      Immediately end the current game and change the map
    help            Prints this message or the help of the given subcommand(s)
    kick            Kick player off server
    list_maps       List the maps on the server
    maps            Show the current and next map
    players         List the players on the server
    raw             Send a raw command to the server
    set_next_map    Set the next map to play on the server
    squads          List the squads on the server
    teams           List the teams on the server
```