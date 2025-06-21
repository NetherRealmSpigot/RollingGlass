use std::io::{stdout, Write};

use clap::{ArgAction, command, Parser};

use crate::protocol::{DEFAULT_PORT, MINECRAFT_1_8, ping, ProtocolNum};

mod protocol;
mod packet;
mod dns;

#[derive(Parser)]
#[command(about = "Minecraft Server List Ping tool", long_about = None, disable_help_flag = true)]
struct RollingGlassArguments {
    #[arg(long = "help", action = ArgAction::Help, help = "Print this help information")]
    _help: Option<bool>,
    #[arg(short, long, help = "Server host")]
    host: String,
    #[arg(long)]
    fakehost: Option<String>,
    #[arg(short, long, default_value_t = DEFAULT_PORT, help = "Server port")]
    port: u16,
    #[arg(long, default_value_t = MINECRAFT_1_8, help = "Protocol number")]
    protocol: ProtocolNum,
    #[arg(long, default_value_t = 5u8, help = "Connection timeout in seconds")]
    timeout: u8
}

#[tokio::main]
async fn main() {
    let args: RollingGlassArguments = RollingGlassArguments::parse();
    let fakehost_arg = args.fakehost;
    let fakehost;
    match fakehost_arg {
        Some(v) => { fakehost = v; }
        None => { fakehost = String::new(); }
    }
    let res = ping(&args.host, &args.port, &fakehost, &args.protocol, &args.timeout).await.expect("Failed to ping");
    let _ = stdout().write_all(&res);
}
