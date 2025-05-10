use sysinfo::{Process, System};

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: i32,
    pub name: String,
    pub cmd: Vec<String>,
    pub exe: String,
}

pub fn get_all_processes() -> Vec<ProcessInfo> {
    let mut system = System::new_all();
    system.refresh_all();

    system
        .processes()
        .values()
        .map(|process| ProcessInfo {
            pid: process.pid().as_u32() as i32,
            name: process.name().to_string(),
            cmd: process.cmd().to_vec(),
            exe: process
                .exe()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "".to_string()),
        })
        .collect()
}
