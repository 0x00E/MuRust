use super::*;
use muonline_packet::{Packet, PacketDecodable, PacketType};

/// An aggregation of all possible client packets.
#[derive(Debug)]
pub enum Client {
  ClientTime(ClientTime),
  AccountLoginRequest(AccountLoginRequest),
  CharacterListRequest,
  CharacterCreate(CharacterCreate),
  CharacterDelete(CharacterDelete),
  CharacterJoinRequest(CharacterJoinRequest),
  None,
}

impl Client {
  /// Constructs a client packet from an unidentified one.
  pub fn from_packet(packet: &Packet) -> io::Result<Self> {
    // TODO: Box the largest packets to decrease total size?
    // TODO: Handle this boilerplate, subcodes should also be automatic
    match (packet.code(), packet.data()) {
      (ClientTime::CODE, &[0x00, _..]) => ClientTime::from_packet(packet).map(Client::ClientTime),
      (AccountLoginRequest::CODE, &[0x01, _..]) => {
        AccountLoginRequest::from_packet(packet).map(Client::AccountLoginRequest)
      },
      (CharacterListRequest::CODE, &[0x00, _..]) => {
        CharacterListRequest::from_packet(packet).map(|_| Client::CharacterListRequest)
      },
      (CharacterCreate::CODE, &[0x01, _..]) => {
        CharacterCreate::from_packet(packet).map(Client::CharacterCreate)
      },
      (CharacterDelete::CODE, &[0x02, _..]) => {
        CharacterDelete::from_packet(packet).map(Client::CharacterDelete)
      },
      (CharacterJoinRequest::CODE, &[0x03, _..]) => {
        CharacterJoinRequest::from_packet(packet).map(Client::CharacterJoinRequest)
      },
      _ => Ok(Client::None),
    }
  }
}
