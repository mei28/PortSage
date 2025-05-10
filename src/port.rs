use std::collections::HashMap;
use std::process::Command;

/// creathe a map of port to PID from lsof output
pub fn parse_lsof_output(output: &str) -> HashMap<u16, u32> {
    let mut map = HashMap::new();

    for line in output.lines().skip(1) {
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() < 9 {
            continue;
        }

        let pid: u32 = cols[1].parse().unwrap_or(0);
        let nameport = cols[8];
        if let Some(port_str) = nameport.split(':').last() {
            if let Ok(port) = port_str.parse::<u16>() {
                map.insert(port, pid);
            }
        }
    }

    map
}

/// get the port to PID map at runtime
pub fn get_port_pid_map() -> HashMap<u16, u32> {
    let output = Command::new("lsof")
        .args(&["-iTCP", "-sTCP:LISTEN", "-nP"])
        .output()
        .expect("failed to execute lsof");

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_lsof_output(&stdout)
}
