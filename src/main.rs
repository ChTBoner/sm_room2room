mod room_data;
mod supermetroid;
mod usb2snes;

use room_data::room_data::Room;
use supermetroid::super_metroid::{GameInfo, GameState, GameStates, GameTime};
use time::{Duration, Instant};
use usb2snes::usb2snes::SyncClient;

fn main() {
    let mut usb2snes = SyncClient::connect();

    println!("Connected to the Usb2snes server");
    usb2snes.set_name(String::from("usb2snes-cli"));
    println!("Server version is : {:?}", usb2snes.app_version());

    let devices = usb2snes.list_device();

    usb2snes.attach(&devices[0]);

    let game_info = GameInfo::new();

    let mut previous_state = GameState::new();
    let mut global_timer = Instant::now();
    let mut room_timer = Instant::now();

    let mut previous_game_time = GameTime::new();
    let mut game_time = GameTime::new();

    let mut prev_room_name = Room::new();
    let mut current_room = Room::new();

    let mut global_rta_room_entered;

    loop {
        let current_state = game_info.get_game_state(&mut usb2snes);
        if previous_state != current_state {
            dbg!(&current_state);
        }
        match current_state.state {
            GameStates::NewGame => {
                global_timer = Instant::now();
                prev_room_name = Room::new();
                previous_game_time = GameTime::new();
                game_time = GameTime::new();
                current_room = Room {
                    smile_id: "0xDF45".to_string(),
                    region: "Ceres".to_string(),
                    subregion: "".to_string(),
                    room_name: "Ceres Elevator Room".to_string(),
                };
            }
            GameStates::Playing | GameStates::Saving => {
                if [
                    GameStates::DoorTransition,
                    GameStates::CeresElevator,
                    GameStates::NewGame,
                ]
                .contains(&previous_state.state)
                {
                    current_room = game_info.get_room_info(&mut usb2snes);
                    global_rta_room_entered = global_timer.elapsed();
                    println!(
                        "Entering {} from {}",
                        &current_room.room_name, &prev_room_name.room_name
                    );
                    println!("{}", global_rta_room_entered);
                    game_time.print_game_time();
                }
            }
            GameStates::DoorTransition | GameStates::CeresElevator => {
                if previous_state.state != current_state.state {
                    game_time = game_info.get_game_time(&mut usb2snes);
                    let game_time_spent_in_room = GameTime::diff(&game_time, &previous_game_time);
                    let real_time_spent_in_room = room_timer.elapsed();
                    println!("Leaving {}", &current_room.room_name);
                    println!("RTA = {}", real_time_spent_in_room,);
                    println!(
                        "IGT = {}{}f",
                        game_time_spent_in_room.total, game_time_spent_in_room.frames
                    );
                    prev_room_name = current_room.to_owned();
                    previous_game_time = game_time.clone();
                    room_timer = Instant::now();
                }
            }
            GameStates::GameTimeEnd => {
                game_time = game_info.get_game_time(&mut usb2snes);
                println!(
                    "Game Time => {}h{}m{}s{}f",
                    game_time.hours, game_time.minutes, game_time.seconds, game_time.frames
                );
            }
            _ => {
                if previous_state != current_state {
                    println!("Not Playing");
                }
            }
        };
        if previous_state != current_state {
            previous_state = current_state.clone()
        }
    }
}
