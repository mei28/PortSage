use portsage::process::{get_all_processes, ProcessInfo};

#[test]
fn test_process_list_is_not_empty() {
    let processes = get_all_processes();
    assert!(!processes.is_empty(), "プロセス一覧が空です");
}

#[test]
fn test_contains_current_shell_or_cargo() {
    let processes = get_all_processes();
    let has_expected = processes.iter().any(|proc| {
        let name = proc.name.to_lowercase();
        name.contains("bash")
            || name.contains("zsh")
            || name.contains("cargo")
            || name.contains("rustc")
    });

    assert!(has_expected, "bash/zsh/cargoなどのプロセスが見つかりませんでした");
}

#[test]
fn test_process_fields_are_valid() {
    let processes = get_all_processes();
    for proc in processes {
        assert!(!proc.name.is_empty(), "プロセス名が空です (PID: {})", proc.pid);
        assert!(proc.pid > 0, "PIDが無効です: {}", proc.pid);
        // cmdやexeは空の可能性があるが、チェックすることも可
    }
}
