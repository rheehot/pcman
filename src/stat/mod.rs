use anyhow::Error;

#[cfg(not(target_os = "macos"))]
pub fn init_tray() -> Result<(), Error> {
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn init_tray() -> Result<(), Error> {
    use sysbar::Sysbar;
    let mut bar = Sysbar::new("pcman");

    bar.add_item(
        "Say 'bar'",
        Box::new(move || {
            println!("bar");
        }),
    );

    bar.add_quit_item("Quit");

    bar.display();

    Ok(())
}
