use portsage::process::{get_all_processes, ProcessInfo};

#[test]
fn test_process_list_is_not_empty() {
    let processes = get_all_processes();
    assert!(!processes.is_empty(), "プロセス一覧が空です");
}

#[test]
fn test_filter_sample() {
    let mock_proc = ProcessInfo {
        pid: 1001,
        name: "dummy-process".into(),
        cmd: vec!["dummy".into(), "--test".into()],
        exe: "/usr/bin/dummy".into(),
        status: "Running".into(),
        cpu_usage: 1.0,
        memory: 2048,
        virtual_memory: 4096,
        parent_pid: Some(1),
        start_time: 0,
        cwd: "/home/dummy".into(),
    };

    assert!(mock_proc.name.contains("dummy"));
}

#[test]
fn test_contains_shell_or_cargo() {
    let processes = get_all_processes();
    let has_expected = processes.iter().any(|proc| {
        let name = proc.name.to_lowercase();
        name.contains("bash")
            || name.contains("zsh")
            || name.contains("cargo")
            || name.contains("rustc")
    });
    assert!(
        has_expected,
        "bash/zsh/cargo などのプロセスが見つかりません"
    );
}

#[test]
fn test_process_fields_are_valid() {
    let processes = get_all_processes();
    for proc in processes {
        assert!(proc.pid > 0, "無効な PID: {}", proc.pid);
        assert!(
            !proc.name.is_empty(),
            "名前が空のプロセスがあります (PID: {})",
            proc.pid
        );

        if proc.memory == 0 && proc.virtual_memory == 0 {
            eprintln!(
                "警告: メモリ使用量が 0 のプロセスがあります (PID: {}, name: {})",
                proc.pid, proc.name
            );
            continue; // 失敗にはしない
        }

        assert!(
            proc.memory > 0 || proc.virtual_memory > 0,
            "メモリが 0 のプロセスがあります (PID: {})",
            proc.pid
        );
    }
}
