mod qusb2snes;
mod roomdata;
mod supermetroid;

use clearscreen::clear;
use qusb2snes::usb2snes::SyncClient;
use roomdata::room_data::Room;
use supermetroid::super_metroid::{GameInfo, GameStates, GameTime};
use time::Instant;

fn clear_term() {
    // print!("{}[2J", 27 as char);
    clear().expect("failed to clear screen");
}
fn main() {
    let mut usb2snes = SyncClient::connect();
    clear_term();
    println!("Connected to the Usb2snes server");
    usb2snes.set_name(String::from("usb2snes-cli"));
    println!("Server version is : {:?}", usb2snes.app_version());

    let devices = usb2snes.list_device();

    usb2snes.attach(&devices[0]);

    let mut game_info = GameInfo::new();

    // let mut previous_state = GameState::new();
    let mut global_timer = Instant::now();

    // let mut previous_game_time = GameTime::new();
    // let mut game_time = GameTime::new();

    // let mut prev_room_name = Room::new();
    let mut previous_room = Room::new(
        vec![0, 0],
        global_timer.elapsed(),
        game_info.current_game_time,
    );
    let mut current_room = Room::new(
        vec![0, 0],
        global_timer.elapsed(),
        game_info.current_game_time,
    );

    let mut global_rta_room_entered;

    loop {
        game_info.update_data(&mut usb2snes);
        // let current_state = game_info.get_game_state(&mut usb2snes);
        // if game_info.previous_game_state != game_info.current_game_state {
        //     dbg!(&game_info.current_game_state);
        // }
        match game_info.current_game_state {
            GameStates::NewGame => {
                // reset global timer
                global_timer = Instant::now();

                // reset IGT info
                game_info.previous_game_time = GameTime::new();
                game_info.current_game_time = GameTime::new();

                // reset room info
                previous_room = Room::new(
                    vec![0, 0],
                    global_timer.elapsed(),
                    game_info.current_game_time,
                );
                current_room = Room::new(
                    vec![0, 0],
                    global_timer.elapsed(),
                    game_info.current_game_time,
                );
            }
            GameStates::Playing | GameStates::Saving => {
                if [
                    GameStates::DoorTransition,
                    GameStates::CeresElevator,
                    GameStates::NewGame,
                    GameStates::ProgramStarted,
                    GameStates::LoadGame,
                    GameStates::Elevator,
                ]
                .contains(&game_info.previous_game_state)
                {
                    // entering new room
                    previous_room = current_room;
                    current_room = Room::new(
                        game_info.current_room_id.to_owned(),
                        global_timer.elapsed(),
                        game_info.current_game_time,
                    );

                    //get current game time for next comparison
                    game_info.previous_game_time = game_info.current_game_time;
                    global_rta_room_entered = global_timer.elapsed();
                    // println!(
                    //     "Entering {} from {}",
                    //     current_room.location.name, previous_room.location.name
                    // );
                    // println!("{}", global_rta_room_entered);
                }
                // if game_info.previous_game_state == GameStates::RealTimeEnd {
                //     global_timer.
                // }
            }
            GameStates::DoorTransition | GameStates::CeresElevator | GameStates::Elevator => {
                if game_info.previous_game_state != game_info.current_game_state {
                    let igt_in_room = GameTime::diff(
                        game_info.current_game_time.to_owned(),
                        current_room.igt_entry,
                    );
                    let rta_in_room = global_timer.elapsed() - current_room.rta_entry;
                    clear_term();
                    println!("{}", &current_room.location.name);
                    println!("RTA = {}", rta_in_room,);
                    igt_in_room.print_game_time();
                }
            }
            GameStates::RealTimeEnd => {
                if game_info.current_game_state != game_info.previous_game_state {
                    clear_term();
                    println!("Run finished - RTA : {}", global_timer.elapsed());
                }
            }
            GameStates::GameTimeEnd => {
                if game_info.current_game_state != game_info.previous_game_state {
                    println!("IGT Run finished");
                    game_info.current_game_time.print_game_time();
                }
            }
            GameStates::GameOver | GameStates::Dead => {
                clear_term();
                println!("GameOver");
                let total_rta = global_timer.elapsed();
                println!("RTA: {}", total_rta);
                game_info.current_game_time.print_game_time();
            }
            _ => {
                // if game_info.previous_game_state != game_info.current_game_state {
                //     println!("Not Playing - {}", game_info.current_game_state);
                // clear();
                // println!("Not Playing - {}", game_info.current_game_state);
                // }
            }
        };
        if game_info.previous_game_state != game_info.current_game_state {
            game_info.previous_game_state = game_info.current_game_state
        }
    }
}
