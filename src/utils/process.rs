use std::process;

use sysinfo::{Pid, ProcessExt, System, SystemExt};

pub fn current_total_memory_usage(system: &mut System) -> Option<(u64, u64)> {
    let current_process = process::id();
    let current_pid = Pid::from(current_process as i32);
    system.refresh_process(current_pid);

    let process = system.process(current_pid)?;
    let mem = process.memory();

    let children_memory_usage: u64 = process.tasks.values().map(|v| v.memory()).sum();

    Some((mem, children_memory_usage))
}
