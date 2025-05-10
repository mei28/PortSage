use clap::Parser;
use portsage::cli::Cli;

#[test]
fn test_parse_filter_argument() {
    let args = vec!["test", "--filter", "uvicorn"];
    let cli = Cli::parse_from(args);
    assert_eq!(cli.filter.as_deref(), Some("uvicorn"));
}

#[test]
fn test_parse_port_argument() {
    let args = vec!["test", "--port", "8080"];
    let cli = Cli::parse_from(args);
    assert_eq!(cli.port, Some(8080));
}

#[test]
fn test_parse_all_arguments() {
    let args = vec![
        "test", "--filter", "node", "--port", "3000", "--json", "--kill", "123",
    ];
    let cli = Cli::parse_from(args);
    // assert!(cli.json);
    assert_eq!(cli.kill, Some(123));
}
