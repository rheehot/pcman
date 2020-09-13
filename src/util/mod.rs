use anyhow::Error;
use notify_rust::Notification;

pub fn notify_kill(msg: &str) -> Result<(), Error> {
    Notification::new()
        .appname("pcman")
        .summary("Killed process")
        .body(msg)
        .timeout(5)
        .show()?;

    Ok(())
}
