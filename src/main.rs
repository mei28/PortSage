use portsage::cli::Cli;
use portsage::process::get_all_processes;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    let all_procs = get_all_processes();
    for proc in all_procs {
        println!(
            "[PID {}] {} => {:?}",
            proc.pid,
            proc.name,
            proc.cmd
        );
    }
}

