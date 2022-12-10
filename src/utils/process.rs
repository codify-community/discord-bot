use std::process;

use sysinfo::{Pid, ProcessExt, System, SystemExt};

pub fn me(system: &mut System) -> Option<(f32, u64)> {
    let current_process = process::id();
    let current_pid = Pid::from(current_process as i32);
    system.refresh_process(current_pid);

    let process = system.process(current_pid)?;

    Some((process.cpu_usage(), process.memory()))
}
