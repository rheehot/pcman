use anyhow::{Context, Error};
use std::collections::HashMap;
use sysinfo::{Pid, Process, ProcessExt, Signal, System, SystemExt};

fn pid_of_rust_analyzer(processes: &HashMap<Pid, Process>) -> Result<Vec<(&Pid, &Process)>, Error> {
    let mut ret = vec![];
    for (pid, v) in processes {
        let name = v.name();

        if name.starts_with("rust-analyzer") {
            ret.push((pid, v));
        }
    }

    Ok(ret)
}

fn kill_rustc_if_required(processes: &HashMap<Pid, Process>, parent: Pid) -> Result<(), Error> {
    let cpu_count = num_cpus::get_physical();

    let mut rustc_processes: Vec<&Process> = processes
        .iter()
        .filter_map(|(.., process)| {
            if process.name() != "rustc" {
                return None;
            }

            // We only care about rustc spawned by `parent`
            if let Some(parent_pid) = process.parent() {
                if parent == parent_pid {
                    return Some(process);
                }
            }

            None
        })
        .collect();

    if cpu_count * 2 >= rustc_processes.len() {
        return Ok(());
    }
    rustc_processes.sort_by_key(|p| p.start_time());

    for i in 0..(rustc_processes.len() - cpu_count * 2) {
        let p = rustc_processes[i];
        p.kill(Signal::Interrupt);
        eprintln!("Killed rustc process {}", p.pid())
    }

    Ok(())
}

/// Kills `rustc` automatically if rust-analyzer spawns too much.
pub fn kill_if_required(system: &System) -> Result<(), Error> {
    let processes = system.get_processes();

    let rust_analyzers =
        pid_of_rust_analyzer(processes).context("failed to get pid of rust-analyzer")?;

    for (pid, ..) in rust_analyzers {
        kill_rustc_if_required(processes, *pid).context("failed to kill rustc")?;
    }

    Ok(())
}
