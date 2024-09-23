use clap::Parser;
use reqwest::Client;
use std::env;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    interface: Option<String>,
    #[arg(short, long)]
    target_server: Option<String>,
}

#[tokio::main]
async fn main() {
    // Argument list using clap
    let args = Args::parse();

    // Try if we get something from env variables
    let interface_env = env::var("SENDMYIP_INTERFACE").ok();
    let target_host_env = env::var("SENDMYIP_TARGET_HOST").ok();

    // Determine the interface to use
    let interface = if let Some(interface) = args.interface {
        interface
    } else if let Some(interface) = interface_env {
        interface
    } else {
        "eth0".to_string()
    };

    // Determine the target host to used
    let target_host = if let Some(target) = args.target_server {
        target
    } else if let Some(target) = target_host_env {
        target
    } else {
        "http://127.0.0.1:8087/ip".to_string()
    };

    let ip = sendmyip::get_ipv4_address(&interface);
    let ipv6 = sendmyip::get_ipv6_address(&interface);
    let hostname = sendmyip::get_hostname();
    let map = sendmyip::gather_info("dummy address", &interface);
    println!("IPv4 address: {}", ip);
    println!("IPv6 address: {}", ipv6);
    println!("Hostname: {}", hostname);
    println!("The HashMap with all the info: {:?}", map.clone());

    // Let's try to POST that stuff with Reqwest

    let client = Client::new();
    let res = client.post(&target_host).json(&map).send().await;
    println!("Response: {:?}", res);
}
