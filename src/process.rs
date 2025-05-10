use std::collections::HashMap;
use sysinfo::{PidExt, ProcessExt, System, SystemExt};
use tabled::Tabled;

use crate::port::get_port_pid_map;

#[derive(Clone)]
pub struct ProcessInfo {
    pub pid: i32,
    pub name: String,
    pub cmd: Vec<String>,
    pub exe: String,
    pub status: String,
    pub cpu_usage: f32,
    pub memory: u64,
    pub virtual_memory: u64,
    pub parent_pid: Option<i32>,
    pub start_time: u64,
    pub cwd: String,
    pub ports: Vec<u16>,
}

#[derive(Tabled)]
pub struct DisplayProcessInfo {
    pub pid: i32,
    pub name: String,
    pub ports: String,
    pub command: String,
}

impl From<&ProcessInfo> for DisplayProcessInfo {
    fn from(p: &ProcessInfo) -> Self {
        DisplayProcessInfo {
            pid: p.pid,
            name: p.name.clone(),
            ports: p
                .ports
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            command: p.cmd.join(" "),
        }
    }
}

pub fn get_all_processes() -> Vec<ProcessInfo> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let port_map: HashMap<u16, u32> = get_port_pid_map();

    sys.processes()
        .values()
        .map(|p| {
            let pid = p.pid().as_u32() as i32;
            let ports = port_map
                .iter()
                .filter_map(|(port, mapped_pid)| {
                    if *mapped_pid as i32 == pid {
                        Some(*port)
                    } else {
                        None
                    }
                })
                .collect::<Vec<u16>>();

            ProcessInfo {
                pid,
                name: p.name().to_string(),
                cmd: p.cmd().to_vec(),
                exe: p.exe().display().to_string(),
                status: format!("{:?}", p.status()),
                cpu_usage: p.cpu_usage(),
                memory: p.memory(),
                virtual_memory: p.virtual_memory(),
                parent_pid: p.parent().map(|pid| pid.as_u32() as i32),
                start_time: p.start_time(),
                cwd: p.cwd().display().to_string(),
                ports,
            }
        })
        .collect()
}
