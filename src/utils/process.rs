use std::process;
use sysinfo::{Pid, ProcessExt, System, SystemExt};

#[tracing::instrument(skip_all)]
pub fn me(system: &mut System) -> Option<(f32, u64, u64)> {
    let current_process = process::id();
    let current_pid = Pid::from(current_process as i32);
    system.refresh_process(current_pid);
    system.refresh_processes();

    let mut subprocesses = 0;

    for (id, proc) in system.processes() {
        if proc.parent() == Some(current_pid) {
            subprocesses += proc.memory();
            tracing::trace!(
                "Found process: {id:?} @ {:?} using {} KiB",
                proc.exe(),
                subprocesses / 1024
            );
        }
    }

    let process = system.process(current_pid)?;

    Some((process.cpu_usage(), process.memory(), subprocesses))
}
