use crate::Error;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

pub const SERVERDATA_AUTH: i32 = 3;
pub const SERVERDATA_AUTH_RESPONSE: i32 = 2;
pub const SERVERDATA_EXECCOMMAND: i32 = 2;
pub const SERVERDATA_RESPONSE_VALUE: i32 = 0;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RconPacket {
    id: i32,
    type_: i32,
    body: String,
}

impl RconPacket {
    pub fn new<S: Into<String>>(id: i32, type_: i32, body: S) -> RconPacket {
        let body: String = body.into();

        RconPacket {
            id: id,
            type_: type_,
            body: body,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn type_(&self) -> i32 {
        self.type_
    }
    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut data = Vec::new();
        data.write_i32::<LittleEndian>((self.body().len() + 10) as i32)?;
        data.write_i32::<LittleEndian>(self.id())?;
        data.write_i32::<LittleEndian>(self.type_())?;
        data.append(
            &mut self
                .body()
                .as_bytes()
                .into_iter()
                .map(|b| *b)
                .collect::<Vec<u8>>(),
        );
        data.push(0);
        data.push(0);
        Ok(data)
    }

    pub fn into_body(self) -> String {
        self.body
    }
}

pub struct RconClient {
    next_id: i32,
    password: String,
    stream: TcpStream,
}

impl RconClient {
    pub fn connect<A: ToSocketAddrs, S: Into<String>>(
        addr: A,
        password: S,
    ) -> Result<RconClient, Error> {
        let mut rcon_client = RconClient {
            next_id: 0,
            password: password.into(),
            stream: TcpStream::connect(addr)?,
        };
        rcon_client.authenticate()?;
        Ok(rcon_client)
    }

    pub fn authenticate(&mut self) -> Result<(), Error> {
        let authentication_packet =
            RconPacket::new(self.get_next_id(), SERVERDATA_AUTH, self.password());

        let response_packet = self.send_and_get_response(&authentication_packet)?;
        if response_packet.type_() != SERVERDATA_RESPONSE_VALUE {
            return Err(Error::ProtocolError);
        }

        let response_packet = self.recv_packet()?;
        if response_packet.type_() != SERVERDATA_AUTH_RESPONSE {
            return Err(Error::ProtocolError);
        }

        if response_packet.id() == -1 {
            return Err(Error::AuthenticationFailure);
        }

        Ok(())
    }

    pub fn send_packet(&mut self, packet: &RconPacket) -> Result<(), Error> {
        self.stream.write_all(&packet.encode()?)?;
        Ok(())
    }

    pub fn recv_packet(&mut self) -> Result<RconPacket, Error> {
        let size = self.stream.read_i32::<LittleEndian>()?;
        let id = self.stream.read_i32::<LittleEndian>()?;
        let type_ = self.stream.read_i32::<LittleEndian>()?;

        let mut body = vec![0u8; size as usize - 10];
        self.stream.read_exact(&mut body)?;

        let mut null_bytes = vec![0u8; 2];
        self.stream.read_exact(&mut null_bytes)?;

        let body = String::from_utf8(body)?;

        let packet = RconPacket::new(id, type_, body);

        Ok(packet)
    }

    pub fn send_and_get_response(&mut self, packet: &RconPacket) -> Result<RconPacket, Error> {
        self.send_packet(packet)?;
        self.recv_packet()
    }

    pub fn get_next_id(&mut self) -> i32 {
        let next_id = self.next_id;
        self.next_id += 1;
        next_id
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn exec_command<S: Into<String>>(&mut self, command: S) -> Result<String, Error> {
        let mut body_parts: Vec<String> = Vec::new();

        let request_id = self.get_next_id();
        let chk_id = self.get_next_id();

        let request_packet = RconPacket::new(request_id, SERVERDATA_EXECCOMMAND, command.into());

        // We just do this to make sure we've received all the data in the current packet
        let check_packet = RconPacket::new(chk_id, SERVERDATA_EXECCOMMAND, "ListCommands");

        self.send_packet(&request_packet)?;
        self.send_packet(&check_packet)?;

        loop {
            let response = self.recv_packet()?;

            if response.id() == request_id {
                body_parts.push(response.into_body());
            } else {
                if response.id() == chk_id {
                    break;
                }
            }
        }

        Ok(body_parts.join(""))
    }
}
