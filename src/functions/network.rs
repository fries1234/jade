use crate::internal::*;

pub fn set_hostname(hostname: &str) {
    println!("Setting hostname to {}", hostname);
    files::create_file("/etc/hostname");
    let return_val = files::append_file("/etc/hostname", hostname);
    match return_val {
        Ok(_) => {
            info(format!("Set hostname to {}", hostname));
        }
        Err(e) => {
            crash(
                format!("Failed to set hostname to {}, Error: {}", hostname, e),
                1,
            );
        }
    }
}

pub fn enable_ipv6(ipv6: bool) {
    if ipv6 {
        println!("Enabling IPv6");
        let return_val = files::append_file("/etc/hosts", "::1 localhost");
        match return_val {
            Ok(_) => {
                info("Enabled IPv6".to_string());
            }
            Err(e) => {
                crash(format!("Failed to enable IPv6, Error: {}", e), 1);
            }
        }
    } else {
        println!("Not enabling ipv6");
    }
}
