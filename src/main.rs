use clap::Parser;
use portsage::tui::run_tui;
use portsage::{
    cli::Cli, filter::filter_processes_by_name, port::get_port_pid_map, process::get_all_processes,
    process::DisplayProcessInfo,
};
use tabled::settings::Style;
use tabled::Table;

fn main() {
    let cli = Cli::parse();
    let processes = get_all_processes();

    if !cli.cli {
        run_tui(&processes).unwrap();
        return;
    }

    let mut filtered = processes.clone();

    if let Some(ref keyword) = cli.filter {
        filtered = filter_processes_by_name(&filtered, keyword)
            .into_iter()
            .cloned()
            .collect();
    }

    if let Some(port) = cli.port {
        let port_map = get_port_pid_map();
        if let Some(&pid) = port_map.get(&port) {
            filtered = filtered
                .into_iter()
                .filter(|p| p.pid == pid as i32)
                .collect();
        } else {
            filtered.clear();
        }
    }

    let display_procs: Vec<DisplayProcessInfo> = filtered.iter().map(Into::into).collect();
    let table = Table::new(display_procs).with(Style::rounded()).to_string();

    println!("{table}");
}
