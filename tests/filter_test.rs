use portsage::filter::filter_processes_by_name;
use portsage::process::ProcessInfo;

fn mock_process(pid: i32, name: &str, cmd: &[&str]) -> ProcessInfo {
    ProcessInfo {
        pid,
        name: name.to_string(),
        cmd: cmd.iter().map(|s| s.to_string()).collect(),
        exe: "/usr/bin/dummy".into(),
        status: "Running".into(),
        cpu_usage: 0.0,
        memory: 1024,
        virtual_memory: 2048,
        parent_pid: Some(1),
        start_time: 0,
        cwd: "/tmp".into(),
    }
}

#[test]
fn test_filter_by_name() {
    let processes = vec![
        mock_process(1, "uvicorn", &["uvicorn", "main:app"]),
        mock_process(2, "node", &["node", "index.js"]),
        mock_process(3, "python3", &["python3", "server.py"]),
    ];

    let filtered = filter_processes_by_name(&processes, "uvicorn");
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].pid, 1);

    let filtered2 = filter_processes_by_name(&processes, "python");
    assert_eq!(filtered2.len(), 1);
    assert_eq!(filtered2[0].pid, 3);

    let filtered3 = filter_processes_by_name(&processes, "notfound");
    assert_eq!(filtered3.len(), 0);
}
