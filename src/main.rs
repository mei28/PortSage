use clap::Parser;
use portsage::tui::run_tui;
use portsage::tui::theme::{Theme, KANAGAWA};
use portsage::{
    cli::Cli, filter::filter_processes_by_name, port::get_port_pid_map, process::get_all_processes,
    process::DisplayProcessInfo,
};
use tabled::settings::Style;
use tabled::Table;

/// CLI flag > PORTSAGE_THEME env var > Kanagawa default.
/// Unknown names exit with a non-zero status rather than silently falling
/// back, matching the project's fail-fast policy.
fn resolve_theme(cli_arg: Option<&str>) -> Result<Theme, String> {
    let requested = cli_arg
        .map(String::from)
        .or_else(|| std::env::var("PORTSAGE_THEME").ok());
    match requested {
        Some(name) => Theme::from_name(&name)
            .ok_or_else(|| format!("unknown theme: {name} (expected kanagawa, tokyonight, or nord)")),
        None => Ok(KANAGAWA),
    }
}

fn main() {
    let cli = Cli::parse();
    let processes = get_all_processes();

    if !cli.cli {
        let theme = match resolve_theme(cli.theme.as_deref()) {
            Ok(t) => t,
            Err(msg) => {
                eprintln!("error: {msg}");
                std::process::exit(2);
            }
        };
        run_tui(&processes, &theme).unwrap();
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
