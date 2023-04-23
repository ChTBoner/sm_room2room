mod roomdata;
mod supermetroid;

use clearscreen::clear;
use rusb2snes::SyncClient;
use tungstenite::Error;

use supermetroid::super_metroid::GameInfo;

fn clear_term() {
    clear().expect("failed to clear screen");
}

fn main() -> Result<(), Error> {
    let mut usb2snes = SyncClient::connect()?;
    clear_term();
    println!("Connected to {}", usb2snes.app_version()?);

    usb2snes.set_name(String::from("sm_room2room"))?;

    let devices = usb2snes.list_device()?;

    usb2snes.attach(&devices[0])?;
    let info = usb2snes.info()?;
    println!("Attached to {} - {}", info.dev_type, info.version);

    let mut game_info = GameInfo::new();

    loop {
        game_info.update_state(&mut usb2snes);
        // info!("{}", game_info.log_data);
    }
}
