use crate::process::ProcessInfo;

pub fn filter_processes_by_name<'a>(
    processes: &'a [ProcessInfo],
    keyword: &str,
) -> Vec<&'a ProcessInfo> {
    let keyword_lower = keyword.to_lowercase();
    processes
        .iter()
        .filter(|p| {
            p.name.to_lowercase().contains(&keyword_lower)
                || p.cmd
                    .iter()
                    .any(|arg| arg.to_lowercase().contains(&keyword_lower))
        })
        .collect()
}
