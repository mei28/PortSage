use crate::process::ProcessInfo;

pub fn apply_filter(processes: &[ProcessInfo], keyword: &str) -> Vec<ProcessInfo> {
    let keyword = keyword.to_lowercase();
    processes
        .iter()
        .filter(|p| {
            p.pid.to_string().contains(&keyword)
                || p.name.to_lowercase().contains(&keyword)
                || p.cmd.iter().any(|c| c.to_lowercase().contains(&keyword))
        })
        .cloned()
        .collect()
}
