mod room_data;
mod usb2snes;
mod supermetroid;

use usb2snes::usb2snes::SyncClient;
use std::time::{Duration, Instant};
use supermetroid::super_metroid::{GameInfo, GameStates};
use room_data::room_data::Room;


fn main() {
    let mut usb2snes = SyncClient::connect();

    println!("Connected to the Usb2snes server");
    usb2snes.set_name(String::from("usb2snes-cli"));
    println!("Server version is : {:?}", usb2snes.app_version());

    let devices = usb2snes.list_device();

    usb2snes.attach(&devices[0]);

    let mut prev_room_name = Room::new();

    let mut previous_state = &GameStates::Unknown;
    // let now = Instant::now();
    let game_info = GameInfo::new();
    loop {
        let current_state = game_info.get_game_state(&mut usb2snes);
        
        let current_room = game_info.get_room_info(&mut usb2snes);


        if prev_room_name != current_room {
            prev_room_name = current_room.clone();
            dbg!(current_room);
        }

        if previous_state != current_state {
            previous_state = current_state;
            dbg!(current_state);
        }
    }
}
