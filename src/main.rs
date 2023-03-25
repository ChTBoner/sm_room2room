mod qusb2snes;
mod roomdata;
mod supermetroid;

use clearscreen::clear;
use qusb2snes::usb2snes::SyncClient;
use roomdata::room_data::Room;
use supermetroid::super_metroid::{GameInfo, GameStates, GameTime};

fn clear_term() {
    clear().expect("failed to clear screen");
}

fn main() {
    let mut usb2snes = SyncClient::connect();
    clear_term();
    println!("Connected to {:?}", usb2snes.app_version());

    usb2snes.set_name(String::from("sm_room2room"));

    let devices = usb2snes.list_device();

    usb2snes.attach(&devices[0]);

    let mut game_info = GameInfo::new();

    loop {
        game_info.update_data(&mut usb2snes);

        match game_info.current_game_state {
            GameStates::NewGame => {
                if game_info.previous_game_state != game_info.current_game_state {
                    game_info.reset();
                }
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
                    game_info.previous_room = game_info.current_room.clone();
                    game_info.current_room = Room::new(
                        game_info.current_room.id,
                        game_info.global_timer.elapsed(),
                        game_info.current_game_time,
                    );

                    //get current game time for next comparison
                    game_info.previous_game_time = game_info.current_game_time;
                }
            }
            GameStates::DoorTransition | GameStates::CeresElevator | GameStates::Elevator => {
                if game_info.previous_game_state != game_info.current_game_state {
                    let igt_in_room = GameTime::diff(
                        game_info.current_game_time.to_owned(),
                        game_info.current_room.igt_entry,
                    );
                    let rta_in_room =
                        game_info.global_timer.elapsed() - game_info.current_room.rta_entry;
                    clear_term();
                    println!("{}", &game_info.current_room.location.name);
                    println!("RTA = {}", rta_in_room,);
                    igt_in_room.print_game_time();
                }
            }
            GameStates::RealTimeEnd => {
                if game_info.current_game_state != game_info.previous_game_state {
                    clear_term();
                    println!("Run finished - RTA : {}", game_info.global_timer.elapsed());
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
                let total_rta = game_info.global_timer.elapsed();
                println!("RTA: {}", total_rta);
                game_info.current_game_time.print_game_time();
            }
            _ => {}
        };
        if game_info.previous_game_state != game_info.current_game_state {
            game_info.previous_game_state = game_info.current_game_state
        }
    }
}
