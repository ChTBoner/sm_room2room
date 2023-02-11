pub mod super_metroid {
    use std::{collections::HashMap};
    use crate::usb2snes;
    use crate::room_data::room_data;

    use crate::room_data::room_data::Room;
    #[derive(Debug, PartialEq)]
    pub enum GameStates {
        Logo,           // 0x00
        TitleScreen,    // 0x01 - 1
        OptionMode,     // 0x02 - 2
        Menus,          // 0x04 - 4
        LoadArea,       // 0x05 - 5
        LoadGame,       // 0x06
        Saving,         // 0x07
        Playing,        // 0x08
        DoorTransition, 
        Unpausing,      // 0x10, 0x11, 0x12
        Pausing,        // 0x0C, 0x0D, 0x0E
        Paused,         // 0x0F
        Dead,           // 0x15, 0x17, 0x18, 0x19, 0x1A
        TimerUp,        // 0x23
        GameOver,       // 0x24
        Demos,          // 0x2A
        Unknown
    }

    #[derive(Debug)]
    pub struct Address{
        pub location: u32,
        pub size: usize,
    }

    #[derive(Debug)]
    pub struct GameInfo {
        pub rooms_data: HashMap<String, Room>,
        pub current_room_address: Address,
        pub game_state_address: Address,
        pub game_states: HashMap<u8, GameStates>,
    }

    impl GameInfo {
        pub fn new() -> Self {
            Self { 
                rooms_data: room_data::room_data_gen(),
                current_room_address: Address{
                    location: 0xF5079B,
                    size: 2,
                },
                game_state_address: Address{
                    location: 0xF50998,
                    size: 1,
                },
                game_states: HashMap::from([
                    (0x00, GameStates::Logo),
                    (0x01, GameStates::TitleScreen),
                    (0x02, GameStates::OptionMode),
                    (0x04, GameStates::Menus),
                    (0x05, GameStates::LoadArea),
                    (0x06, GameStates::LoadGame),
                    (0x07, GameStates::Saving),
                    (0x08, GameStates::Playing),
                    (0x09, GameStates::DoorTransition),
                    (0x0B, GameStates::DoorTransition),
                    (0x0C, GameStates::Pausing),
                    (0x0D, GameStates::Pausing),
                    (0x0E, GameStates::Pausing),
                    (0x10, GameStates::Unpausing),
                    (0x11, GameStates::Unpausing),
                    (0x12, GameStates::Unpausing),
                    (0x0F, GameStates::Paused),
                    (0x13, GameStates::Dead),
                    (0x14, GameStates::Dead),
                    (0x15, GameStates::Dead),
                    (0x16, GameStates::Dead),
                    (0x17, GameStates::Dead),
                    (0x18, GameStates::Dead),
                    (0x19, GameStates::Dead),
                    (0x1A, GameStates::Dead),
                    (0x23, GameStates::TimerUp),
                    (0x24, GameStates::GameOver),
                    (0x25, GameStates::GameOver),
                    (0x28, GameStates::Demos),
                    (0x29, GameStates::Demos),
                    (0x2A, GameStates::Demos),
                    (0x2B, GameStates::Demos),
                    (0x2C, GameStates::Demos),

                ])

            }
        }

        pub fn get_game_state(&self, client: &mut usb2snes::usb2snes::SyncClient) -> &GameStates {
            let result = client.get_address(
                self.game_state_address.location, 
                self.game_state_address.size
            );
            
            self.game_states.get(&result[0]).unwrap()
        }

        pub fn get_room_info(&self, client: &mut usb2snes::usb2snes::SyncClient) -> Room {
            let result = client.get_address(
                self.current_room_address.location,
                self.current_room_address.size
            );    
            
            let key = format!("0x{:02X}{:02X}", result[1], result[0]).to_owned();
            match self.rooms_data.get(&key) {
                Some(room_name) => return room_name.to_owned(),
                None => Room::new()
            }
        }
        
    }
    
    // #[derive(Debug)]
    // pub struct GameData<'a> {
    //     pub current_state: &'a GameStates,
    //     pub current_room: Room,
    // }

    // impl<'a> GameData<'a> {
    //     pub fn new() -> Self {
    //         Self { 
    //             current_state: &GameStates::Unknown,
    //             current_room: Room {
    //                 smile_id: String::from(""),
    //                 room_name: String::from("")
    //             }
    //         }
    //     }
        
    //     pub fn update(self, client: &mut usb2snes::SyncClient, info: &GameInfo) {
    //         self.get_game_state(client, &info)
    //     }

        
}