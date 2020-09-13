use anyhow::Error;
use tray_item::TrayItem;

pub fn init_tray() -> Result<(), Error> {
    let mut tray = TrayItem::new("pcman", "").unwrap();

    tray.add_menu_item("Run now", || {}).unwrap();

    let inner = tray.inner_mut();
    inner.add_quit_item("Quit");
    inner.display();

    Ok(())
}
