pub mod super_metroid {
    use crate::qusb2snes::usb2snes::SyncClient;
    use crate::roomdata::room_data::Room;
    use std::collections::HashMap;
    use strum_macros::Display;
    use time::{Duration, Instant};

    #[derive(Debug, PartialEq, Clone, Copy, Display)]
    pub enum GameStates {
        Logo,        // 0x00
        TitleScreen, // 0x01 - 1
        OptionMode,  // 0x02 - 2
        Menus,       // 0x04 - 4
        LoadArea,    // 0x05 - 5
        LoadGame,    // 0x06
        Saving,      // 0x07
        Playing,     // 0x08
        DoorTransition,
        Unpausing, // 0x10, 0x11, 0x12
        Pausing,   // 0x0C, 0x0D, 0x0E
        Paused,    // 0x0F
        Dead,      // 0x15, 0x17, 0x18, 0x19, 0x1A
        TimerUp,   // 0x23
        GameOver,  // 0x24
        Demos,     // 0x2A
        NewGame,
        CeresEscape,
        CeresElevator,
        Elevator,
        GameTimeEnd,
        RealTimeEnd,
        Credits,
        OpeningSeq,
        Unknown,
        ProgramStarted,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Events {
        AnimalsSaved,
        ZebesAblaze,
        TourianOpen,
        ShaktoolDoneDigging,
        ChozoLoweredAcid,
        MeridiaTubeBroken,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct GameTime {
        pub frames: u8,
        pub seconds: u8,
        pub minutes: u8,
        pub hours: u8,
        pub total: Duration,
    }

    impl GameTime {
        pub fn new() -> Self {
            Self {
                frames: 0,
                seconds: 0,
                minutes: 0,
                hours: 0,
                total: Duration::new(0, 0),
            }
        }

        pub fn diff(current: GameTime, previous: GameTime) -> Self {
            let total_diff = current.total - previous.total;

            let frames = if current.frames >= previous.frames {
                current.frames - previous.frames
            } else {
                60 - previous.frames + current.frames
            };

            Self {
                frames,
                seconds: total_diff.whole_seconds() as u8,
                minutes: total_diff.whole_minutes() as u8,
                hours: total_diff.whole_hours() as u8,
                total: total_diff,
            }
        }

        pub fn print_game_time(self) {
            println!(
                "Game Time => {}h{}m{}s{}f",
                self.hours, self.minutes, self.seconds, self.frames
            );
        }
    }

    #[derive(Debug)]
    pub struct GameInfo {
        pub previous_room: Room,
        pub current_room: Room,

        pub global_timer: Instant,
        pub current_game_time: GameTime,
        pub previous_game_time: GameTime,

        pub event_flags: Vec<Events>,

        pub previous_game_state: GameStates,
        pub current_game_state: GameStates,

        pub game_states: HashMap<u8, GameStates>,
    }

    impl GameInfo {
        pub fn new() -> Self {
            Self {
                // current room data
                previous_room: Room::new(vec![0, 0], Duration::new(0, 0), GameTime::new()),
                current_room: Room::new(vec![0, 0], Duration::new(0, 0), GameTime::new()),

                global_timer: Instant::now(),
                // game time data
                current_game_time: GameTime::new(),
                previous_game_time: GameTime::new(),

                event_flags: Vec::new(),

                // current game state data
                previous_game_state: GameStates::ProgramStarted,
                current_game_state: GameStates::ProgramStarted,

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
                    (0x10, GameStates::DoorTransition),
                    (0x0A, GameStates::DoorTransition),
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
                    (0x1E, GameStates::OpeningSeq),
                    (0x1F, GameStates::NewGame),
                    (0x20, GameStates::CeresElevator),
                    (0x21, GameStates::CeresEscape),
                    (0x22, GameStates::CeresEscape),
                    (0x23, GameStates::TimerUp),
                    (0x24, GameStates::GameOver),
                    (0x25, GameStates::GameOver),
                    (0x26, GameStates::GameTimeEnd),
                    (0x27, GameStates::Credits),
                    (0x28, GameStates::Demos),
                    (0x29, GameStates::Demos),
                    (0x2A, GameStates::Demos),
                    (0x2B, GameStates::Demos),
                    (0x2C, GameStates::Demos),
                ]),
            }
        }
        pub fn reset(&mut self) {
            self.global_timer = Instant::now();
            self.previous_game_time = GameTime::new();
            self.current_game_time = GameTime::new();
            self.previous_room = Room::new(
                vec![0, 0],
                self.global_timer.elapsed(),
                self.current_game_time,
            );
            self.current_room = Room::new(
                vec![0, 0],
                self.global_timer.elapsed(),
                self.current_game_time,
            );
        }
        pub fn update_data(&mut self, client: &mut SyncClient) {
            let addresses_array = [
                (0xF50998, 1), // game state
                (0xF509DA, 8), // game time
                (0xF5079B, 2), // current roomID
                (0xF5D821, 1), // event flag
                (0xF50FB2, 2), // enemy AI to identify ship in RTA
                (0xF50E18, 1), //elevator room transition
            ];

            let data = client.get_addresses(&addresses_array).unwrap();

            self.event_flags = self.get_event_flags(&data[3]);
            self.current_game_time = self.get_game_time(&data[1]);
            self.current_room.id = data[2].to_owned();
            self.current_game_state = self.get_game_state(&data[0], &data[4], &data[5]);
            // dbg!(&data[5]);
        }

        fn get_event_flags(&self, result: &[u8]) -> Vec<Events> {
            // convert u8 to string to represent a binarry number
            let events_array = format!("{:08b}", result[0]);
            // convert the string to a bit array
            let flags_array: Vec<u32> = events_array
                .chars()
                .map(|c| c.to_digit(2).unwrap())
                .collect();

            let mut events: Vec<Events> = Vec::new();

            if flags_array[0] == 1 {
                events.push(Events::AnimalsSaved);
            }
            if flags_array[1] == 1 {
                events.push(Events::ZebesAblaze);
            }
            if flags_array[2] == 1 {
                events.push(Events::ShaktoolDoneDigging);
            }
            if flags_array[3] == 1 {
                events.push(Events::ChozoLoweredAcid);
            }
            if flags_array[4] == 1 {
                events.push(Events::MeridiaTubeBroken);
            }
            if flags_array[5] == 1 {
                events.push(Events::TourianOpen);
            }
            events
        }

        fn get_game_state(&self, result: &[u8], ai: &[u8], elevator: &[u8]) -> GameStates {
            if elevator != &[0] {
                return GameStates::Elevator;
            }

            if self.event_flags.contains(&Events::ZebesAblaze) && ai == [0x4F, 0xAA] {
                return GameStates::RealTimeEnd;
            };

            let id = result[0].to_owned();
            match self.game_states.get(&id) {
                Some(state) => state.to_owned(),
                None => {
                    println!("unknown game state: {:02X}", id);
                    GameStates::Unknown
                }
            }
        }

        pub fn get_game_time(&self, result: &[u8]) -> GameTime {
            GameTime {
                frames: result[0],
                seconds: result[2],
                minutes: result[4],
                hours: result[6],
                total: Duration::seconds(result[2] as i64)
                    + Duration::minutes(result[4] as i64)
                    + Duration::hours(result[6] as i64),
            }
        }
    }
}
