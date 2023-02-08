use std::collections::HashMap;

mod room_data;
mod usb2snes;

fn get_room_name(rooms_data: &HashMap<String, String>, curr_room_id: Vec<u8>) -> String {
    let key = format!("0x{:02X}{:02X}", curr_room_id[1], curr_room_id[0]).to_owned();
    match rooms_data.get(&key) {
        Some(room_name) => return room_name.to_owned(),
        None => format!("{}, Room Name Unknown", key),
    }
}

fn main() {
    let rooms_data = room_data::room_data::room_data_gen();

    let mut usb2snes = usb2snes::usb2snes::SyncClient::connect();

    println!("Connected to the Usb2snes server");
    usb2snes.set_name(String::from("usb2snes-cli"));
    println!("Server version is : {:?}", usb2snes.app_version());

    let devices = usb2snes.list_device();
    dbg!(&devices);

    usb2snes.attach(&devices[0]);

    let mut prev_room_name = "".to_string();
    loop {
        let curr_room_id = usb2snes.get_address(0xF5079B, 2);

        let current_room_name = get_room_name(&rooms_data, curr_room_id);

        if prev_room_name != current_room_name {
            println!("{}", current_room_name);
            prev_room_name = current_room_name.to_owned();
        }
    }
}
