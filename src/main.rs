use anyhow::{bail, Error};
use daemonize::Daemonize;
use sysinfo::{System, SystemExt};

mod stat;
mod supports;
mod util;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let system = System::new_all();
    supports::rust_analyzer::kill_if_required(&system)?;

    let daemonize = Daemonize::new()
        .pid_file("/tmp/pcman.pid")
        .chown_pid_file(true)
        .working_directory("/tmp")
        .exit_action(|| eprintln!("Terminating..."));

    match daemonize.start() {
        Ok(_) => println!("Success, daemonized"),
        Err(e) => bail!("Error, {}", e),
    }

    Ok(())
}
