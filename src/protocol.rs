#![allow(unused)]

use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::slice;
use std::sync::OnceLock;
use std::time::Duration;

use im::hashset::HashSet;

use crate::dns::resolve;
use crate::packet::{compose_handshake_packet, compose_status_request_packet};

pub type ProtocolNum = u16;

pub const DEFAULT_PORT: u16 = 25565;

pub const MINECRAFT_1_7: ProtocolNum = 3;
pub const MINECRAFT_1_7_1: ProtocolNum = 3;

pub const MINECRAFT_1_7_2: ProtocolNum = 4;
pub const MINECRAFT_1_7_3: ProtocolNum = 4;
pub const MINECRAFT_1_7_4: ProtocolNum = 4;
pub const MINECRAFT_1_7_5: ProtocolNum = 4;

pub const MINECRAFT_1_7_6: ProtocolNum = 5;
pub const MINECRAFT_1_7_7: ProtocolNum = 5;
pub const MINECRAFT_1_7_8: ProtocolNum = 5;
pub const MINECRAFT_1_7_9: ProtocolNum = 5;
pub const MINECRAFT_1_7_10: ProtocolNum = 5;

pub const MINECRAFT_1_8: ProtocolNum = 47;
pub const MINECRAFT_1_8_1: ProtocolNum = 47;
pub const MINECRAFT_1_8_2: ProtocolNum = 47;
pub const MINECRAFT_1_8_3: ProtocolNum = 47;
pub const MINECRAFT_1_8_4: ProtocolNum = 47;
pub const MINECRAFT_1_8_5: ProtocolNum = 47;
pub const MINECRAFT_1_8_6: ProtocolNum = 47;
pub const MINECRAFT_1_8_7: ProtocolNum = 47;
pub const MINECRAFT_1_8_8: ProtocolNum = 47;
pub const MINECRAFT_1_8_9: ProtocolNum = 47;

pub const MINECRAFT_1_9: ProtocolNum = 107;
pub const MINECRAFT_1_9_1: ProtocolNum = 108;
pub const MINECRAFT_1_9_2: ProtocolNum = 109;
pub const MINECRAFT_1_9_3: ProtocolNum = 110;
pub const MINECRAFT_1_9_4: ProtocolNum = 110;

pub const MINECRAFT_1_10: ProtocolNum = 210;
pub const MINECRAFT_1_10_1: ProtocolNum = 210;
pub const MINECRAFT_1_10_2: ProtocolNum = 210;

pub const MINECRAFT_1_11: ProtocolNum = 315;
pub const MINECRAFT_1_11_1: ProtocolNum = 316;
pub const MINECRAFT_1_11_2: ProtocolNum = 316;

pub const MINECRAFT_1_12: ProtocolNum = 335;
pub const MINECRAFT_1_12_1: ProtocolNum = 338;
pub const MINECRAFT_1_12_2: ProtocolNum = 340;

pub const MINECRAFT_1_13: ProtocolNum = 393;
pub const MINECRAFT_1_13_1: ProtocolNum = 401;
pub const MINECRAFT_1_13_2: ProtocolNum = 404;

pub const MINECRAFT_1_14: ProtocolNum = 477;
pub const MINECRAFT_1_14_1: ProtocolNum = 480;
pub const MINECRAFT_1_14_2: ProtocolNum = 485;
pub const MINECRAFT_1_14_3: ProtocolNum = 490;
pub const MINECRAFT_1_14_4: ProtocolNum = 498;

pub const MINECRAFT_1_15: ProtocolNum = 573;
pub const MINECRAFT_1_15_1: ProtocolNum = 575;
pub const MINECRAFT_1_15_2: ProtocolNum = 578;

pub const MINECRAFT_1_16: ProtocolNum = 735;
pub const MINECRAFT_1_16_1: ProtocolNum = 736;
pub const MINECRAFT_1_16_2: ProtocolNum = 751;
pub const MINECRAFT_1_16_3: ProtocolNum = 753;
pub const MINECRAFT_1_16_4: ProtocolNum = 754;
pub const MINECRAFT_1_16_5: ProtocolNum = 754;

pub const MINECRAFT_1_17: ProtocolNum = 755;
pub const MINECRAFT_1_17_1: ProtocolNum = 756;

pub const MINECRAFT_1_18: ProtocolNum = 757;
pub const MINECRAFT_1_18_1: ProtocolNum = 757;
pub const MINECRAFT_1_18_2: ProtocolNum = 758;

pub const MINECRAFT_1_19: ProtocolNum = 759;
pub const MINECRAFT_1_19_1: ProtocolNum = 760;
pub const MINECRAFT_1_19_2: ProtocolNum = 760;
pub const MINECRAFT_1_19_3: ProtocolNum = 761;
pub const MINECRAFT_1_19_4: ProtocolNum = 762;

pub const MINECRAFT_1_20: ProtocolNum = 763;
pub const MINECRAFT_1_20_1: ProtocolNum = 763;
pub const MINECRAFT_1_20_2: ProtocolNum = 764;
pub const MINECRAFT_1_20_3: ProtocolNum = 765;
pub const MINECRAFT_1_20_4: ProtocolNum = 765;
pub const MINECRAFT_1_20_5: ProtocolNum = 766;
pub const MINECRAFT_1_20_6: ProtocolNum = 766;

pub const MINECRAFT_1_21: ProtocolNum = 767;
pub const MINECRAFT_1_21_1: ProtocolNum = 767;
pub const MINECRAFT_1_21_2: ProtocolNum = 768;
pub const MINECRAFT_1_21_3: ProtocolNum = 768;
pub const MINECRAFT_1_21_4: ProtocolNum = 769;
pub const MINECRAFT_1_21_5: ProtocolNum = 770;
pub const MINECRAFT_1_21_6: ProtocolNum = 771;
pub const MINECRAFT_1_21_7: ProtocolNum = 772;

pub fn get_known_protocol_numbers() -> &'static HashSet<ProtocolNum> {
    static KNOWN_PROTOCOL_NUMBERS: OnceLock<HashSet<ProtocolNum>> = OnceLock::new();
    KNOWN_PROTOCOL_NUMBERS.get_or_init(|| {
        let mut set = HashSet::new();
        set.insert(MINECRAFT_1_7_1);
        set.insert(MINECRAFT_1_7_5);
        set.insert(MINECRAFT_1_7_10);
        set.insert(MINECRAFT_1_8_9);
        set.insert(MINECRAFT_1_9);
        set.insert(MINECRAFT_1_9_1);
        set.insert(MINECRAFT_1_9_2);
        set.insert(MINECRAFT_1_9_4);
        set.insert(MINECRAFT_1_10_2);
        set.insert(MINECRAFT_1_11);
        set.insert(MINECRAFT_1_11_2);
        set.insert(MINECRAFT_1_12);
        set.insert(MINECRAFT_1_12_1);
        set.insert(MINECRAFT_1_12_2);
        set.insert(MINECRAFT_1_13);
        set.insert(MINECRAFT_1_13_1);
        set.insert(MINECRAFT_1_13_2);
        set.insert(MINECRAFT_1_14);
        set.insert(MINECRAFT_1_14_1);
        set.insert(MINECRAFT_1_14_2);
        set.insert(MINECRAFT_1_14_3);
        set.insert(MINECRAFT_1_14_4);
        set.insert(MINECRAFT_1_15);
        set.insert(MINECRAFT_1_15_1);
        set.insert(MINECRAFT_1_15_2);
        set.insert(MINECRAFT_1_16);
        set.insert(MINECRAFT_1_16_1);
        set.insert(MINECRAFT_1_16_2);
        set.insert(MINECRAFT_1_16_3);
        set.insert(MINECRAFT_1_16_5);
        set.insert(MINECRAFT_1_17);
        set.insert(MINECRAFT_1_17_1);
        set.insert(MINECRAFT_1_18_1);
        set.insert(MINECRAFT_1_18_2);
        set.insert(MINECRAFT_1_19);
        set.insert(MINECRAFT_1_19_2);
        set.insert(MINECRAFT_1_19_3);
        set.insert(MINECRAFT_1_19_4);
        set.insert(MINECRAFT_1_20_1);
        set.insert(MINECRAFT_1_20_2);
        set.insert(MINECRAFT_1_20_4);
        set.insert(MINECRAFT_1_20_6);
        set.insert(MINECRAFT_1_21_1);
        set.insert(MINECRAFT_1_21_3);
        set.insert(MINECRAFT_1_21_4);
        set.insert(MINECRAFT_1_21_5);
        set.insert(MINECRAFT_1_21_6);
        set.insert(MINECRAFT_1_21_7);
        set
    })
}

pub fn is_known_protocol_number(n: &ProtocolNum) -> bool {
    get_known_protocol_numbers().contains(n)
}

pub async fn ping(host: &String, port: &u16, fakehost: &String, protocol: &ProtocolNum, timeout: &u8) -> Result<Vec<u8>, String> {
    if host.is_empty() {
        return Err("Invalid host string".to_string());
    }
    if !is_known_protocol_number(protocol) {
        return Err("Unknown protocol number".to_string());
    }
    if *timeout == 0 {
        return Err("Timeout in seconds must be bigger than 0".to_string());
    }

    let host_touse = if fakehost.is_empty() {
        String::from(host)
    } else {
        String::from(fakehost)
    };

    let port_touse = if *port == 0 {
        DEFAULT_PORT
    } else {
        *port
    };

    let ips = resolve(host.as_str(), &port_touse).await?;

    for v in ips.iter() {
        let dur = Duration::from_secs(*timeout as u64);

        let stream = TcpStream::connect_timeout(&SocketAddr::from(*v), dur);
        if stream.is_err() {
            continue
        }
        let mut stream = stream.unwrap();
        stream.set_read_timeout(Some(dur));
        stream.set_write_timeout(Some(dur));

        stream.write_all(&compose_handshake_packet(&host_touse, &port_touse, protocol)).expect("Cannot perform handshake");
        stream.write_all(&compose_status_request_packet()).expect("Cannot send status request");

        let mut byte = 0u8;

        let resize = read_varint(&mut stream);
        if resize.is_err() {
            return Err(resize.err().unwrap().to_string());
        }

        stream.read_exact(slice::from_mut(&mut byte)).expect("Cannot read packet ID");
        if byte != 0u8 {
            shutoff(&stream);
            return Err(format!("Unknown packet ID {byte}"));
        }

        let resize = read_varint(&mut stream);
        if resize.is_err() {
            return Err(resize.err().unwrap().to_string());
        }
        let resize = resize.unwrap();
        let mut res: Vec<u8> = Vec::new();
        let mut buf = [0; 4096];
        let mut bytes_read = 0;
        while bytes_read < resize {
            let read_res = stream.read(&mut buf);
            if read_res.is_err() {
                shutoff(&stream);
                return Err(read_res.err().unwrap().to_string());
            }
            let read_res = read_res.unwrap();
            bytes_read += read_res;
            res.extend_from_slice(&buf[..read_res]);
        }
        shutoff(&stream);
        return Ok(res);
    }
    Err("All IP addresses tried".to_string())
}

fn read_varint(stream: &mut TcpStream) -> Result<usize, &'static str> {
    let mut byte = 0x00;
    let mut res = 0i32;
    for i in 0.. {
        if i > 5 {
            shutoff(stream);
            return Err("Malformed response, not a valid varint");
        }
        let buf = slice::from_mut(&mut byte);
        #[allow(clippy::unused_io_amount)]
        stream.read(buf).expect("Cannot read varint");
        if buf.is_empty() {
            break;
        }
        res |= (((buf[0] as i32) & 0x7Fi32) << (7 * i));
        if ((buf[0] as i32) & 0x80i32) == 0 {
            break
        }
    }
    if res <= 0 {
        shutoff(stream);
        return Err("Malformed response, varint not bigger than 0");
    }
    Ok(res as usize)
}

fn shutoff(stream: &TcpStream) {
    stream.shutdown(Shutdown::Both);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_protocol_number() {
        assert!(!is_known_protocol_number(&(MINECRAFT_1_7 - 1)));
        assert!(is_known_protocol_number(&MINECRAFT_1_7));
        assert!(is_known_protocol_number(&MINECRAFT_1_7_1));
        assert!(is_known_protocol_number(&MINECRAFT_1_7_2));
        assert!(is_known_protocol_number(&MINECRAFT_1_7_3));
        assert!(is_known_protocol_number(&MINECRAFT_1_7_4));
        assert!(is_known_protocol_number(&MINECRAFT_1_7_5));
        assert!(is_known_protocol_number(&MINECRAFT_1_7_6));
        assert!(is_known_protocol_number(&MINECRAFT_1_7_7));
        assert!(is_known_protocol_number(&MINECRAFT_1_7_8));
        assert!(is_known_protocol_number(&MINECRAFT_1_7_9));
        assert!(is_known_protocol_number(&MINECRAFT_1_7_10));
        assert!(is_known_protocol_number(&MINECRAFT_1_8));
        assert!(is_known_protocol_number(&MINECRAFT_1_8_1));
        assert!(is_known_protocol_number(&MINECRAFT_1_8_2));
        assert!(is_known_protocol_number(&MINECRAFT_1_8_3));
        assert!(is_known_protocol_number(&MINECRAFT_1_8_4));
        assert!(is_known_protocol_number(&MINECRAFT_1_8_5));
        assert!(is_known_protocol_number(&MINECRAFT_1_8_6));
        assert!(is_known_protocol_number(&MINECRAFT_1_8_7));
        assert!(is_known_protocol_number(&MINECRAFT_1_8_8));
        assert!(is_known_protocol_number(&MINECRAFT_1_8_9));
        assert!(is_known_protocol_number(&MINECRAFT_1_9));
        assert!(is_known_protocol_number(&MINECRAFT_1_9_1));
        assert!(is_known_protocol_number(&MINECRAFT_1_9_2));
        assert!(is_known_protocol_number(&MINECRAFT_1_9_3));
        assert!(is_known_protocol_number(&MINECRAFT_1_9_4));
        assert!(is_known_protocol_number(&MINECRAFT_1_10));
        assert!(is_known_protocol_number(&MINECRAFT_1_10_1));
        assert!(is_known_protocol_number(&MINECRAFT_1_10_2));
        assert!(is_known_protocol_number(&MINECRAFT_1_11));
        assert!(is_known_protocol_number(&MINECRAFT_1_11_1));
        assert!(is_known_protocol_number(&MINECRAFT_1_11_2));
        assert!(is_known_protocol_number(&MINECRAFT_1_12));
        assert!(is_known_protocol_number(&MINECRAFT_1_12_1));
        assert!(is_known_protocol_number(&MINECRAFT_1_12_2));
        assert!(is_known_protocol_number(&MINECRAFT_1_13));
        assert!(is_known_protocol_number(&MINECRAFT_1_13_1));
        assert!(is_known_protocol_number(&MINECRAFT_1_13_2));
        assert!(is_known_protocol_number(&MINECRAFT_1_14));
        assert!(is_known_protocol_number(&MINECRAFT_1_14_1));
        assert!(is_known_protocol_number(&MINECRAFT_1_14_2));
        assert!(is_known_protocol_number(&MINECRAFT_1_14_3));
        assert!(is_known_protocol_number(&MINECRAFT_1_14_4));
        assert!(is_known_protocol_number(&MINECRAFT_1_15));
        assert!(is_known_protocol_number(&MINECRAFT_1_15_1));
        assert!(is_known_protocol_number(&MINECRAFT_1_15_2));
        assert!(is_known_protocol_number(&MINECRAFT_1_16));
        assert!(is_known_protocol_number(&MINECRAFT_1_16_1));
        assert!(is_known_protocol_number(&MINECRAFT_1_16_2));
        assert!(is_known_protocol_number(&MINECRAFT_1_16_3));
        assert!(is_known_protocol_number(&MINECRAFT_1_16_4));
        assert!(is_known_protocol_number(&MINECRAFT_1_16_5));
        assert!(is_known_protocol_number(&MINECRAFT_1_17));
        assert!(is_known_protocol_number(&MINECRAFT_1_17_1));
        assert!(is_known_protocol_number(&MINECRAFT_1_18));
        assert!(is_known_protocol_number(&MINECRAFT_1_18_1));
        assert!(is_known_protocol_number(&MINECRAFT_1_18_2));
        assert!(is_known_protocol_number(&MINECRAFT_1_19));
        assert!(is_known_protocol_number(&MINECRAFT_1_19_1));
        assert!(is_known_protocol_number(&MINECRAFT_1_19_2));
        assert!(is_known_protocol_number(&MINECRAFT_1_19_3));
        assert!(is_known_protocol_number(&MINECRAFT_1_19_4));
        assert!(is_known_protocol_number(&MINECRAFT_1_20));
        assert!(is_known_protocol_number(&MINECRAFT_1_20_1));
        assert!(is_known_protocol_number(&MINECRAFT_1_20_2));
        assert!(is_known_protocol_number(&MINECRAFT_1_20_3));
        assert!(is_known_protocol_number(&MINECRAFT_1_20_4));
        assert!(is_known_protocol_number(&MINECRAFT_1_20_5));
        assert!(is_known_protocol_number(&MINECRAFT_1_20_6));
        assert!(is_known_protocol_number(&MINECRAFT_1_21));
        assert!(is_known_protocol_number(&MINECRAFT_1_21_1));
        assert!(is_known_protocol_number(&MINECRAFT_1_21_2));
        assert!(is_known_protocol_number(&MINECRAFT_1_21_3));
        assert!(is_known_protocol_number(&MINECRAFT_1_21_4));
        assert!(is_known_protocol_number(&MINECRAFT_1_21_5));
        assert!(is_known_protocol_number(&MINECRAFT_1_21_6));
        assert!(is_known_protocol_number(&MINECRAFT_1_21_7));
        assert!(!is_known_protocol_number(&(MINECRAFT_1_21_7 + 1)));
    }
}
