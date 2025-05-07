/*
Copyright (c) 2023 Ade M Ramdani <qcynaut@gmail.com>

This software is proprietary and licensed to MyRTS under the terms of the Closed-Source Software License for Freelancers, which is available at https://dictionary.cambridge.org/us/dictionary/english/license.

MyRTS owns all right, title, and interest in and to the software, including all intellectual property rights therein.
MyRTS may use the software for any purpose, including commercial use.
MyRTS may modify the software, but only for their own internal use.
MyRTS may not distribute the software or any modified versions of the software to third parties.
MyRTS may not reverse engineer the software.
MyRTS may not create derivative works from the software.

MyRTS agrees to credit you as the developer of the software in all promotional materials and documentation for the software.

If MyRTS violates any of these terms, their license to use the software will automatically terminate.
*/

use std::collections::HashMap;
use systemstat::{Platform, System};
use types::proto::AvsInfo;

pub mod services;
pub mod states;

/// Get os specific info.
pub fn get_os_info() -> AvsInfo {
    let mut info = AvsInfo {
        networks: None,
        mem_total: None,
        mem_free: None,
        disk_total: None,
        disk_free: None,
        cpu_temp: None,
    };

    let sys = System::new();

    if let Ok(networks) = sys.networks() {
        let mut map: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();
        for (_, network) in networks {
            let mut map2: Vec<HashMap<String, String>> = Vec::new();
            for addr in network.addrs {
                let mut map3: HashMap<String, String> = HashMap::new();
                let ip = match addr.addr {
                    systemstat::IpAddr::Empty => "".to_string(),
                    systemstat::IpAddr::Unsupported => "".to_string(),
                    systemstat::IpAddr::V4(ip) => ip.to_string(),
                    systemstat::IpAddr::V6(ip) => ip.to_string(),
                };
                map3.insert("ip".to_string(), ip);
                let netmask = match addr.netmask {
                    systemstat::IpAddr::Empty => "".to_string(),
                    systemstat::IpAddr::Unsupported => "".to_string(),
                    systemstat::IpAddr::V4(ip) => ip.to_string(),
                    systemstat::IpAddr::V6(ip) => ip.to_string(),
                };
                map3.insert("netmask".to_string(), netmask);
                map2.push(map3);
            }
            map.insert(network.name, map2);
        }
        if let Ok(networks) = serde_json::to_string(&map) {
            info.networks = Some(networks);
        }
    }

    if let Ok(mem) = sys.memory() {
        let total = format!("{}", mem.total.as_u64());
        let free = format!("{}", mem.free.as_u64());
        info.mem_total = Some(total);
        info.mem_free = Some(free);
    }

    if let Ok(disk) = sys.mount_at("/") {
        let total = format!("{}", disk.total.as_u64());
        let free = format!("{}", disk.free.as_u64());
        info.disk_total = Some(total);
        info.disk_free = Some(free);
    }

    if let Ok(temp) = sys.cpu_temp() {
        let temp = format!("{}", temp);
        info.cpu_temp = Some(temp);
    }

    info
}
