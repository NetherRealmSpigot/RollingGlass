#![allow(unused)]

use flashlight::create_varint;

use crate::protocol::ProtocolNum;

fn packet_raw_content_add_varint_length(packet_content: &mut Vec<u8>) {
    packet_content.splice(0..0 , create_varint(&(packet_content.len() as i32)));
}

pub fn compose_handshake_packet(host: &String, port: &u16, protocol: &ProtocolNum) -> Vec<u8> {
    let host_strlen = host.len();
    let mut host_strlen_varint = create_varint(&(host_strlen as i32));
    let host_strlen_varint_bytelen = host_strlen_varint.len();

    let mut protocol_as_varint = create_varint(&(*protocol as i32));
    let protocol_varint_bytelen = protocol_as_varint.len();

    let handshake_content_len = 1 /* Packet ID */ + protocol_varint_bytelen + host_strlen_varint_bytelen + host_strlen + 2 + 1 /* Port number. State number. */;
    let mut handshake_content_len_as_varint = create_varint(&(handshake_content_len as i32));
    let mut content: Vec<u8> = Vec::with_capacity(handshake_content_len + handshake_content_len_as_varint.len());

    content.append(&mut handshake_content_len_as_varint);
    content.push(0x00);
    content.append(&mut protocol_as_varint);
    content.append(&mut host_strlen_varint);
    content.append(&mut host.as_bytes().to_vec());
    content.append(&mut port.to_be_bytes().to_vec());
    content.push(0x01);

    content
}

pub fn compose_status_request_packet() -> Vec<u8> {
    // Packet full length in varint. Packet ID.
    vec![0x01, 0x00]
}

#[cfg(test)]
mod tests {
    use crate::protocol::{MINECRAFT_1_21_6, MINECRAFT_1_8};

    use super::*;

    #[test]
    fn test_add_varint_length() {
        let mut t: Vec<u8> = vec![0x00, 0x01];
        packet_raw_content_add_varint_length(&mut t);
        assert_eq!(vec![0x02, 0x00, 0x01], t);

        let mut t: Vec<u8> = vec![0x01; 255];
        packet_raw_content_add_varint_length(&mut t);
        let mut e: Vec<u8> = vec![255, 1];
        e.append(vec![0x01; 255].as_mut());
        assert_eq!(e, t);

        let mut t: Vec<u8> = vec![0x01; 25565];
        packet_raw_content_add_varint_length(&mut t);
        let mut e: Vec<u8> = vec![221, 199, 1];
        e.append(vec![0x01; 25565].as_mut());
        assert_eq!(e, t);

        let mut t: Vec<u8> = vec![0x01; 2097151];
        packet_raw_content_add_varint_length(&mut t);
        let mut e: Vec<u8> = vec![255, 255, 127];
        e.append(vec![0x01; 2097151].as_mut());
        assert_eq!(e, t);
    }

    #[test]
    fn test_compose_handshake_packet() {
        let mut t: Vec<u8> = vec![0x00, 47, 9, 0x6C, 0x6F, 0x63, 0x61, 0x6C, 0x68, 0x6F, 0x73, 0x74, 0xFF, 0xFE, 0x01];
        packet_raw_content_add_varint_length(&mut t);
        assert_eq!(t, compose_handshake_packet(&"localhost".to_string(), &65534, &MINECRAFT_1_8));

        let mut t: Vec<u8> = vec![0x00, 131, 6, 14, 0x6D, 0x63, 0x2E, 0x68, 0x79, 0x70, 0x69, 0x78, 0x65, 0x6C, 0x2E, 0x6E, 0x65, 0x74, 0x63, 0xDD, 0x01];
        packet_raw_content_add_varint_length(&mut t);
        assert_eq!(t, compose_handshake_packet(&"mc.hypixel.net".to_string(), &25565, &MINECRAFT_1_21_6));
    }

    #[test]
    fn test_status_request_packet() {
        let mut t: Vec<u8> = vec![0x00];
        packet_raw_content_add_varint_length(&mut t);
        assert_eq!(t, compose_status_request_packet());
    }
}