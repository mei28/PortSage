use clap::Parser;

/// PortSage CLI definition
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub filter: Option<String>,

    #[arg(short, long)]
    pub port: Option<u16>,

    #[arg(long)]
    pub json: bool,

    #[arg(long)]
    pub kill: Option<u32>,

    #[arg(long)]
    pub tui: bool,
}
