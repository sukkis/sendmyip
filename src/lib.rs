use std::net::IpAddr;
use std::process::Command;
use std::str;
use std::collections::HashMap;

pub fn get_hostname() -> String {
    let command = "cat /etc/hostname".to_string();

    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let raw_output = str::from_utf8(&output.stdout).expect("Failed to convert output to str");
        println!("Raw output: {}", raw_output); // Debug print
        if !raw_output.is_empty() {
            raw_output.trim().to_string()
        } else {
            panic!("Hostname not found in /etc/hostname");
        }
    } else {
        panic!("Hostname not found in /etc/hostname");
    }
}

pub fn get_ipv4_address(interface: &str) -> String {
    let command = format!(
        "ifconfig {} | grep 'inet ' | awk -F ' ' '{{print $2}}'",
        interface
    );
    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let raw_output = str::from_utf8(&output.stdout).expect("Failed to convert output to str");
        println!("Raw output: {}", raw_output); // Debug print

        let ip = raw_output.trim().to_string();
        if !ip.is_empty() {
            match ip.parse::<IpAddr>() {
                Ok(IpAddr::V4(_)) => ip,
                Ok(IpAddr::V6(_)) => panic!("Expected IP v4 format but got IP v6 format"),
                Err(_) => panic!("Invalid IP address format"),
            }
        } else {
            panic!("No IPv4 address found");
        }
    } else {
        panic!("Command was not successful");
    }
}

pub fn get_ipv6_address(interface: &str) -> String {
    let command = format!(
        "ifconfig {} | grep 'inet6' | awk -F ' ' '{{print $2}}'",
        interface
    );
    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let raw_output =
            str::from_utf8(&output.stdout).expect("Failed to convert output to string");
        println!("Raw output: {}", raw_output); // Debug print
        let ip = raw_output.trim().to_string();
        if !ip.is_empty() {
            match ip.parse::<IpAddr>() {
                Ok(IpAddr::V6(_)) => ip,
                Ok(IpAddr::V4(_)) => panic!("Expected IP v6 format but got IP v4 format"),
                Err(_) => panic!("Invalid IP address format"),
            }
        } else {
            panic!("No IPv6 address found");
        }
    } else {
        panic!("Command was not successful");
    }
}

pub fn gather_info(address: &str, interface: &str) -> HashMap<String, String> {
    let mut map = HashMap::<String, String>::new();
    let v4 = get_ipv4_address(interface);
    let v6 = get_ipv6_address(interface);
    let hostname = get_hostname();
    println!("To be posted to  {}", &address);   
    map.insert("hostname".to_string(), hostname.to_string());
    map.insert("ip_v4".to_string(), v4.to_string());
    map.insert("ip_v6".to_string(), v6.to_string());
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ipv4_address_valid_interface() {
        // Replace "eth0" with an actual interface name on your system for testing
        let interface = "wlp0s20f3";
        let ip = get_ipv4_address(interface);
        assert!(!ip.is_empty(), "IP address should not be empty");
    }

    #[test]
    #[should_panic(expected = "No IPv4 address found")]
    fn test_get_ipv4_address_invalid_interface() {
        let interface = "invalid_interface";
        get_ipv4_address(interface);
    }

    #[test]
    fn test_get_ipv6_address_valid_interface() {
        // Replace "eth0" with an actual interface name on your system for testing
        let interface = "wlp0s20f3";
        let ip = get_ipv6_address(interface);
        assert!(!ip.is_empty(), "IP address should not be empty");
    }

    #[test]
    #[should_panic(expected = "No IPv6 address found")]
    fn test_get_ipv6_address_invalid_interface() {
        let interface = "invalid_interface";
        get_ipv6_address(interface);
    }

    #[test]
    fn test_get_hostname() {
        let hostname = get_hostname();
        assert!(hostname.len() > 2);
    }

    #[test]
    fn test_gather_info() {
        // Replace "eth0" with an actual interface name on your system for testing
	let interface = "wlp0s20f3";
	let address = "http://127.0.0.1/8087/ip";
	let map = gather_info(address, interface);
	assert!(!map.is_empty());
	assert!(map.len() == 3);
	assert!(map.contains_key("ip_v4"));
	assert!(map.contains_key("ip_v6"));
	assert!(map.contains_key("hostname"));	
    }
}
