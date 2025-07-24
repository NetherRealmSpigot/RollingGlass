#![allow(unused)]

use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
use std::sync::OnceLock;

use hickory_resolver::name_server::TokioConnectionProvider;
use hickory_resolver::Resolver;

use crate::protocol::DEFAULT_PORT;

fn get_resolver() -> &'static Resolver<TokioConnectionProvider> {
    static RESOLVER: OnceLock<Resolver<TokioConnectionProvider>> = OnceLock::new();
    RESOLVER.get_or_init(|| {
       Resolver::builder_tokio().unwrap().build()
    })
}

async fn resolve_ipv4(host: &str) -> Vec<Ipv4Addr> {
    let mut ips = Vec::new();
    let res = Resolver::ipv4_lookup(get_resolver(), host).await;
    if res.is_err() {
        return ips
    }
    let res = res.unwrap();
    res.iter().for_each(|a_rec| {
        ips.push(a_rec.0);
    });
    ips
}

async fn resolve_srv(host: &str) -> Vec<(Ipv4Addr, u16)> {
    let mut ips = Vec::new();
    let res = Resolver::srv_lookup(get_resolver(), format!("_minecraft._tcp.{host}")).await;
    if res.is_err() {
        return ips;
    }
    let res = res.unwrap();
    for srv_rec in res.iter() {
        let res4 = resolve_ipv4(srv_rec.target().to_string().as_str()).await;
        if !res4.is_empty() {
            res4.iter().for_each(|v| {
                ips.push((*v, srv_rec.port()));
            });
        }
    }
    ips
}

pub async fn resolve(host: &str, port: &u16) -> Result<Vec<(Ipv4Addr, u16)>, String> {
    let res = IpAddr::from_str(host);
    if let Ok(res_ip) = res {
        return match res_ip {
            IpAddr::V4(res4) => { Ok(vec![(res4, *port)]) },
            IpAddr::V6(_) => { Err("IPv6 not supported".to_string()) }
        }
    }
    if *port == DEFAULT_PORT {
        let res = resolve_srv(host).await;
        if !res.is_empty() {
            return Ok(res)
        }
    }
    let res4 = resolve_ipv4(host).await;
    let mut res = Vec::new();
    res4.iter().for_each(|v| {
        res.push((*v, *port));
    });
    if res.is_empty() {
        return Err(format!("Cannot resolve \"{host}\""))
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::protocol::DEFAULT_PORT;

    use super::*;

    #[tokio::test]
    async fn test_resolve() {
        let r = resolve("::1", &DEFAULT_PORT).await;
        assert!(r.is_err());
        let r = resolve("nope", &DEFAULT_PORT).await;
        assert!(r.is_err());

        let r = resolve("127.0.0.2", &25562u16).await;
        assert!(r.is_ok());
        assert!(r.unwrap().contains(&(Ipv4Addr::from_str("127.0.0.2").unwrap(), 25562u16)));

        let r = resolve("localhost", &25563u16).await;
        assert!(r.is_ok());
        assert!(r.unwrap().contains(&(Ipv4Addr::from_str("127.0.0.1").unwrap(), 25563u16)));

        let r = resolve("play.cubecraft.net", &DEFAULT_PORT).await;
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(!r.is_empty());
        r.iter().for_each(|v| {
            assert_eq!(v.1, DEFAULT_PORT);
        });

        let r = resolve("critz.gg", &25564u16).await;
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(!r.is_empty());
        r.iter().for_each(|v| {
            assert_eq!(v.1, 25564u16);
        });

        let r = resolve("critz.gg", &DEFAULT_PORT).await;
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(!r.is_empty());
        r.iter().for_each(|v| {
            assert_eq!(v.1, DEFAULT_PORT);
        });

        let r = resolve("mc.hypixel.net", &DEFAULT_PORT).await;
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(!r.is_empty());
        assert!(r.len() > 1);
        r.iter().for_each(|v| {
            assert_eq!(v.1, DEFAULT_PORT);
        });
    }
}
