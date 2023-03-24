pub mod room_data {
    use crate::supermetroid::super_metroid::GameTime;
    use std::collections::HashMap;
    use time::Duration;

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Location {
        pub region: String,
        pub name: String,
    }

    impl Location {
        fn empty() -> Self {
            Self {
                region: "Unknown Region".to_string(),
                name: "Unknow Name".to_string(),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Room {
        pub id: Vec<u8>,
        pub location: Location,
        pub rta_entry: Duration,
        pub igt_entry: GameTime,
    }

    impl Room {
        pub fn new(id: Vec<u8>, rta: Duration, igt: GameTime) -> Self {
            Self {
                id: id.to_owned(),
                location: Self::get_room_data(id),
                rta_entry: rta,
                igt_entry: igt,
            }
        }

        fn get_room_data(id: Vec<u8>) -> Location {
            let rooms_data: HashMap<Vec<u8>, Location> = HashMap::from([
                (
                    vec![0xF8, 0x91],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Landing Site".to_string(),
                    },
                ),
                (
                    vec![0xB3, 0x92],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Gauntlet Entrance".to_string(),
                    },
                ),
                (
                    vec![0xFD, 0x92],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Parlor and Alcatraz".to_string(),
                    },
                ),
                (
                    vec![0xAA, 0x93],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Crateria Power Bomb Room".to_string(),
                    },
                ),
                (
                    vec![0xD5, 0x93],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Crateria Save Room".to_string(),
                    },
                ),
                (
                    vec![0xFE, 0x93],
                    Location {
                        region: "Crateria".to_string(),
                        name: "West Ocean".to_string(),
                    },
                ),
                (
                    vec![0x61, 0x94],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Bowling Alley Path".to_string(),
                    },
                ),
                (
                    vec![0x8C, 0x94],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Crateria Kihunter Room".to_string(),
                    },
                ),
                (
                    vec![0xCC, 0x94],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Forgotten Highway Elevator".to_string(),
                    },
                ),
                (
                    vec![0xFD, 0x94],
                    Location {
                        region: "Crateria".to_string(),
                        name: "East Ocean".to_string(),
                    },
                ),
                (
                    vec![0x52, 0x95],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Forgotten Highway Kago Room".to_string(),
                    },
                ),
                (
                    vec![0x7D, 0x95],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Crab Maze".to_string(),
                    },
                ),
                (
                    vec![0xA8, 0x95],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Forgotten Highway Elbow".to_string(),
                    },
                ),
                (
                    vec![0xD4, 0x95],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Crateria Tube".to_string(),
                    },
                ),
                (
                    vec![0xFF, 0x95],
                    Location {
                        region: "Crateria".to_string(),
                        name: "The Moat".to_string(),
                    },
                ),
                (
                    vec![0x2A, 0x96],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Red Brinstar Elevator Room".to_string(),
                    },
                ),
                (
                    vec![0x5B, 0x96],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Gauntlet Energy Tank Room".to_string(),
                    },
                ),
                (
                    vec![0x8F, 0x96],
                    Location {
                        region: "Crateria".to_string(),
                        name: "West Ocean Bridge".to_string(),
                    },
                ),
                (
                    vec![0xBA, 0x96],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Climb".to_string(),
                    },
                ),
                (
                    vec![0x5C, 0x97],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Pit Room".to_string(),
                    },
                ),
                (
                    vec![0xB5, 0x97],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Blue Brinstar Elevator Room".to_string(),
                    },
                ),
                (
                    vec![0x04, 0x98],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Bomb Torizo Room".to_string(),
                    },
                ),
                (
                    vec![0x79, 0x98],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Flyway".to_string(),
                    },
                ),
                (
                    vec![0xE2, 0x98],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Pre-Map Flyway".to_string(),
                    },
                ),
                (
                    vec![0x0D, 0x99],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Terminator Room".to_string(),
                    },
                ),
                (
                    vec![0x38, 0x99],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Green Brinstar Elevator Room".to_string(),
                    },
                ),
                (
                    vec![0x69, 0x99],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Lower Mushrooms".to_string(),
                    },
                ),
                (
                    vec![0x94, 0x99],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Crateria Map Room".to_string(),
                    },
                ),
                (
                    vec![0xBD, 0x99],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Green Pirates Shaft".to_string(),
                    },
                ),
                (
                    vec![0xF9, 0x99],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Crateria Super Room".to_string(),
                    },
                ),
                (
                    vec![0x44, 0x9A],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Final Missile Bombway".to_string(),
                    },
                ),
                (
                    vec![0x90, 0x9A],
                    Location {
                        region: "Crateria".to_string(),
                        name: "The Final Missile".to_string(),
                    },
                ),
                (
                    vec![0xD9, 0x9A],
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Green Brinstar Main Shaft".to_string(),
                    },
                ),
                (
                    vec![0x5B, 0x9B],
                    Location {
                        region: "Pink Brinstar".to_string(),
                        name: "Spore Spawn Super Room".to_string(),
                    },
                ),
                (
                    vec![0x9D, 0x9B],
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Brinstar Pre-Map Room".to_string(),
                    },
                ),
                (
                    vec![0xC8, 0x9B],
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Early Supers Room".to_string(),
                    },
                ),
                (
                    vec![0x07, 0x9C],
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Brinstar Reserve Tank Room".to_string(),
                    },
                ),
                (
                    vec![0x35, 0x9C],
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Brinstar Map Room".to_string(),
                    },
                ),
                (
                    vec![0x5E, 0x9C],
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Green Brinstar Fireflea Room".to_string(),
                    },
                ),
                (
                    vec![0x89, 0x9C],
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Green Brinstar Missile Station".to_string(),
                    },
                ),
                (
                    vec![0xB3, 0x9C],
                    Location {
                        region: "Pink Brinstar".to_string(),
                        name: "Dachora Room".to_string(),
                    },
                ),
                (
                    vec![0x19, 0x9D],
                    Location {
                        region: "Pink Brinstar".to_string(),
                        name: "Big Pink".to_string(),
                    },
                ),
                (
                    vec![0x9C, 0x9D],
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Spore Spawn Kihunter Room".to_string(),
                    },
                ),
                (
                    vec![0xC7, 0x9D],
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Spore Spawn Room".to_string(),
                    },
                ),
                (
                    vec![0x11, 0x9E],
                    Location {
                        region: "Pink Brinstar".to_string(),
                        name: "Pink Brinstar Power Bomb Room".to_string(),
                    },
                ),
                (
                    vec![0x52, 0x9E],
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Green Hill Zone".to_string(),
                    },
                ),
                (
                    vec![0x9F, 0x9E],
                    Location {
                        region: "Blue Brinstar".to_string(),
                        name: "Morph Ball Room".to_string(),
                    },
                ),
                (
                    vec![0x11, 0x9F],
                    Location {
                        region: "Blue Brinstar".to_string(),
                        name: "Construction Zone".to_string(),
                    },
                ),
                (
                    vec![0x64, 0x9F],
                    Location {
                        region: "Blue Brinstar".to_string(),
                        name: "Blue Brinstar Energy Tank Room".to_string(),
                    },
                ),
                (
                    vec![0xBA, 0x9F],
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Noob Bridge".to_string(),
                    },
                ),
                (
                    vec![0xE5, 0x9F],
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Green Brinstar Beetom Room".to_string(),
                    },
                ),
                (
                    vec![0x11, 0xA0],
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Etecoon Energy Tank Room".to_string(),
                    },
                ),
                (
                    vec![0x51, 0xA0],
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Etecoon Super Room".to_string(),
                    },
                ),
                (
                    vec![0x7B, 0xA0],
                    Location {
                        region: "Pink Brinstar".to_string(),
                        name: "Dachora Energy Charge Station".to_string(),
                    },
                ),
                (
                    vec![0xA4, 0xA0],
                    Location {
                        region: "Pink Brinstar".to_string(),
                        name: "Spore Spawn Farming Room".to_string(),
                    },
                ),
                (
                    vec![0xD2, 0xA0],
                    Location {
                        region: "Pink Brinstar".to_string(),
                        name: "Waterway Energy Tank Room".to_string(),
                    },
                ),
                (
                    vec![0x07, 0xA1],
                    Location {
                        region: "Blue Brinstar".to_string(),
                        name: "First Missile Room".to_string(),
                    },
                ),
                (
                    vec![0x30, 0xA1],
                    Location {
                        region: "Pink Brinstar".to_string(),
                        name: "Pink Brinstar Hopper Room".to_string(),
                    },
                ),
                (
                    vec![0x5B, 0xA1],
                    Location {
                        region: "Pink Brinstar".to_string(),
                        name: "Hopper Energy Tank Room".to_string(),
                    },
                ),
                (
                    vec![0x84, 0xA1],
                    Location {
                        region: "Pink Brinstar".to_string(),
                        name: "Big Pink Save Room".to_string(),
                    },
                ),
                (
                    vec![0xAD, 0xA1],
                    Location {
                        region: "Blue Brinstar".to_string(),
                        name: "Blue Brinstar Boulder Room".to_string(),
                    },
                ),
                (
                    vec![0xD8, 0xA1],
                    Location {
                        region: "Blue Brinstar".to_string(),
                        name: "Billy Mays Room".to_string(),
                    },
                ),
                (
                    vec![0x01, 0xA2],
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Green Brinstar Save Room".to_string(),
                    },
                ),
                (
                    vec![0x2A, 0xA2],
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Etecoon Save Room".to_string(),
                    },
                ),
                (
                    vec![0x53, 0xA2],
                    Location {
                        region: "Red Brinstar".to_string(),
                        name: "Red Tower".to_string(),
                    },
                ),
                (
                    vec![0x93, 0xA2],
                    Location {
                        region: "Red Brinstar".to_string(),
                        name: "Red Brinstar Fireflea Room".to_string(),
                    },
                ),
                (
                    vec![0xCE, 0xA2],
                    Location {
                        region: "Red Brinstar".to_string(),
                        name: "X-Ray Scope Room".to_string(),
                    },
                ),
                (
                    vec![0xF7, 0xA2],
                    Location {
                        region: "Red Brinstar".to_string(),
                        name: "Hellway".to_string(),
                    },
                ),
                (
                    vec![0x22, 0xA3],
                    Location {
                        region: "Red Brinstar".to_string(),
                        name: "Caterpillar Room".to_string(),
                    },
                ),
                (
                    vec![0x7C, 0xA3],
                    Location {
                        region: "Red Brinstar".to_string(),
                        name: "Beta Power Bomb Room".to_string(),
                    },
                ),
                (
                    vec![0xAE, 0xA3],
                    Location {
                        region: "Red Brinstar".to_string(),
                        name: "Alpha Power Bomb Room".to_string(),
                    },
                ),
                (
                    vec![0xDD, 0xA3],
                    Location {
                        region: "Red Brinstar".to_string(),
                        name: "Bat Room".to_string(),
                    },
                ),
                (
                    vec![0x08, 0xA4],
                    Location {
                        region: "Red Brinstar".to_string(),
                        name: "Below Spazer".to_string(),
                    },
                ),
                (
                    vec![0x47, 0xA4],
                    Location {
                        region: "Red Brinstar".to_string(),
                        name: "Spazer Room".to_string(),
                    },
                ),
                (
                    vec![0x71, 0xA4],
                    Location {
                        region: "Warehouse".to_string(),
                        name: "Warehouse Zeela Room".to_string(),
                    },
                ),
                (
                    vec![0xB1, 0xA4],
                    Location {
                        region: "Warehouse".to_string(),
                        name: "Warehouse Energy Tank Room".to_string(),
                    },
                ),
                (
                    vec![0xDA, 0xA4],
                    Location {
                        region: "Warehouse".to_string(),
                        name: "Warehouse Kihunter Room".to_string(),
                    },
                ),
                (
                    vec![0x21, 0xA5],
                    Location {
                        region: "Warehouse".to_string(),
                        name: "Baby Kraid Room".to_string(),
                    },
                ),
                (
                    vec![0x6B, 0xA5],
                    Location {
                        region: "Warehouse".to_string(),
                        name: "Kraid Eye Door Room".to_string(),
                    },
                ),
                (
                    vec![0x9F, 0xA5],
                    Location {
                        region: "Warehouse".to_string(),
                        name: "Kraid Room".to_string(),
                    },
                ),
                (
                    vec![0xED, 0xA5],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Statues Hallway".to_string(),
                    },
                ),
                (
                    vec![0x18, 0xA6],
                    Location {
                        region: "Red Brinstar".to_string(),
                        name: "Sloaters Refill".to_string(),
                    },
                ),
                (
                    vec![0x41, 0xA6],
                    Location {
                        region: "Warehouse".to_string(),
                        name: "Kraid Recharge Stations".to_string(),
                    },
                ),
                (
                    vec![0x6A, 0xA6],
                    Location {
                        region: "Crateria".to_string(),
                        name: "Statues Room".to_string(),
                    },
                ),
                (
                    vec![0xA1, 0xA6],
                    Location {
                        region: "Warehouse".to_string(),
                        name: "Warehouse Entrance".to_string(),
                    },
                ),
                (
                    vec![0xE2, 0xA6],
                    Location {
                        region: "Warehouse".to_string(),
                        name: "Varia Suit Room".to_string(),
                    },
                ),
                (
                    vec![0x0B, 0xA7],
                    Location {
                        region: "Warehouse".to_string(),
                        name: "Warehouse Save Room".to_string(),
                    },
                ),
                (
                    vec![0x34, 0xA7],
                    Location {
                        region: "Red Brinstar".to_string(),
                        name: "Red Brinstar Save Room".to_string(),
                    },
                ),
                (
                    vec![0x5D, 0xA7],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Ice Beam Acid Room".to_string(),
                    },
                ),
                (
                    vec![0x88, 0xA7],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Cathedral".to_string(),
                    },
                ),
                (
                    vec![0xB3, 0xA7],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Cathedral Entrance".to_string(),
                    },
                ),
                (
                    vec![0xDE, 0xA7],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Business Center".to_string(),
                    },
                ),
                (
                    vec![0x15, 0xA8],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Ice Beam Gate Room".to_string(),
                    },
                ),
                (
                    vec![0x65, 0xA8],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Ice Beam Tutorial Room".to_string(),
                    },
                ),
                (
                    vec![0x90, 0xA8],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Ice Beam Room".to_string(),
                    },
                ),
                (
                    vec![0xB9, 0xA8],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Ice Beam Snake Room".to_string(),
                    },
                ),
                (
                    vec![0xF8, 0xA8],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Crumble Shaft".to_string(),
                    },
                ),
                (
                    vec![0x23, 0xA9],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Crocomire Speedway".to_string(),
                    },
                ),
                (
                    vec![0x8D, 0xA9],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Crocomire's Room".to_string(),
                    },
                ),
                (
                    vec![0xE5, 0xA9],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Hi Jump Boots Room".to_string(),
                    },
                ),
                (
                    vec![0x0E, 0xAA],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Crocomire Escape".to_string(),
                    },
                ),
                (
                    vec![0x41, 0xAA],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Hi Jump Energy Tank Room".to_string(),
                    },
                ),
                (
                    vec![0x82, 0xAA],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Post Crocomire Farming Room".to_string(),
                    },
                ),
                (
                    vec![0xB5, 0xAA],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Post Crocomire Save Room".to_string(),
                    },
                ),
                (
                    vec![0xDE, 0xAA],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Post Crocomire Power Bomb Room".to_string(),
                    },
                ),
                (
                    vec![0x07, 0xAB],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Post Crocomire Shaft".to_string(),
                    },
                ),
                (
                    vec![0x3B, 0xAB],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Post Crocomire Missile Room".to_string(),
                    },
                ),
                (
                    vec![0x64, 0xAB],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Grapple Tutorial Room 3".to_string(),
                    },
                ),
                (
                    vec![0x8F, 0xAB],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Post Crocomire Jump Room".to_string(),
                    },
                ),
                (
                    vec![0xD2, 0xAB],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Grapple Tutorial Room 2".to_string(),
                    },
                ),
                (
                    vec![0x00, 0xAC],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Grapple Tutorial Room 1".to_string(),
                    },
                ),
                (
                    vec![0x2B, 0xAC],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Grapple Beam Room".to_string(),
                    },
                ),
                (
                    vec![0x5A, 0xAC],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Norfair Reserve Tank Room".to_string(),
                    },
                ),
                (
                    vec![0x83, 0xAC],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Green Bubbles Missile Room".to_string(),
                    },
                ),
                (
                    vec![0xB3, 0xAC],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Bubble Mountain".to_string(),
                    },
                ),
                (
                    vec![0xF0, 0xAC],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Speed Booster Hall".to_string(),
                    },
                ),
                (
                    vec![0x1B, 0xAD],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Speed Booster Room".to_string(),
                    },
                ),
                (
                    vec![0x5E, 0xAD],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Single Chamber".to_string(),
                    },
                ),
                (
                    vec![0xAD, 0xAD],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Double Chamber".to_string(),
                    },
                ),
                (
                    vec![0xDE, 0xAD],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Wave Beam Room".to_string(),
                    },
                ),
                (
                    vec![0x07, 0xAE],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Spiky Platforms Tunnel".to_string(),
                    },
                ),
                (
                    vec![0x32, 0xAE],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Volcano Room".to_string(),
                    },
                ),
                (
                    vec![0x74, 0xAE],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Kronic Boost Room".to_string(),
                    },
                ),
                (
                    vec![0xB4, 0xAE],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Magdollite Tunnel".to_string(),
                    },
                ),
                (
                    vec![0xDF, 0xAE],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Purple Shaft".to_string(),
                    },
                ),
                (
                    vec![0x14, 0xAF],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Lava Dive Room".to_string(),
                    },
                ),
                (
                    vec![0x3F, 0xAF],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Lower Norfair Elevator".to_string(),
                    },
                ),
                (
                    vec![0x72, 0xAF],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Upper Norfair Farming Room".to_string(),
                    },
                ),
                (
                    vec![0xA3, 0xAF],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Rising Tide".to_string(),
                    },
                ),
                (
                    vec![0xCE, 0xAF],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Acid Snakes Tunnel".to_string(),
                    },
                ),
                (
                    vec![0xFB, 0xAF],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Spiky Acid Snakes Tunnel".to_string(),
                    },
                ),
                (
                    vec![0x26, 0xB0],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Nutella Refill".to_string(),
                    },
                ),
                (
                    vec![0x51, 0xB0],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Purple Farming Room".to_string(),
                    },
                ),
                (
                    vec![0x7A, 0xB0],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Bat Cave".to_string(),
                    },
                ),
                (
                    vec![0xB4, 0xB0],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Norfair Map Room".to_string(),
                    },
                ),
                (
                    vec![0xDD, 0xB0],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Bubble Mountain Save Room".to_string(),
                    },
                ),
                (
                    vec![0x06, 0xB1],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Frog Speedway".to_string(),
                    },
                ),
                (
                    vec![0x39, 0xB1],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Red Pirate Shaft".to_string(),
                    },
                ),
                (
                    vec![0x67, 0xB1],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Frog Savestation".to_string(),
                    },
                ),
                (
                    vec![0x92, 0xB1],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Crocomire Save Room".to_string(),
                    },
                ),
                (
                    vec![0xBB, 0xB1],
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Lower Norfair Elevator Save Room".to_string(),
                    },
                ),
                (
                    vec![0xE5, 0xB1],
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Acid Statue Room".to_string(),
                    },
                ),
                (
                    vec![0x36, 0xB2],
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Main Hall".to_string(),
                    },
                ),
                (
                    vec![0x83, 0xB2],
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Golden Torizo's Room".to_string(),
                    },
                ),
                (
                    vec![0xDA, 0xB2],
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Fast Ripper Room".to_string(),
                    },
                ),
                (
                    vec![0x05, 0xB3],
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Golden Torizo Energy Recharge".to_string(),
                    },
                ),
                (
                    vec![0x2E, 0xB3],
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Ridley's Room".to_string(),
                    },
                ),
                (
                    vec![0x7A, 0xB3],
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Lower Norfair Farming Room".to_string(),
                    },
                ),
                (
                    vec![0xA5, 0xB3],
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Fast Pillars Setup Room".to_string(),
                    },
                ),
                (
                    vec![0xE1, 0xB3],
                    Location {
                        region: "".to_string(),
                        name: "Unused Crocomire Room".to_string(),
                    },
                ),
                (
                    vec![0x0A, 0xB4],
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Mickey Mouse Room".to_string(),
                    },
                ),
                (
                    vec![0x57, 0xB4],
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Pillar Room".to_string(),
                    },
                ),
                (
                    vec![0x82, 0xB4],
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Plowerhouse Room".to_string(),
                    },
                ),
                (
                    vec![0xAD, 0xB4],
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "The Worst Room In The Game".to_string(),
                    },
                ),
                (
                    vec![0xE5, 0xB4],
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Amphitheatre".to_string(),
                    },
                ),
                (
                    vec![0x10, 0xB5],
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Lower Norfair Spring Ball Maze Room".to_string(),
                    },
                ),
                (
                    vec![0x5A, 0xB5],
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Lower Norfair Escape Power Bomb Room".to_string(),
                    },
                ),
                (
                    vec![0x85, 0xB5],
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Red Kihunter Shaft".to_string(),
                    },
                ),
                (
                    vec![0xD5, 0xB5],
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Wasteland".to_string(),
                    },
                ),
                (
                    vec![0x2B, 0xB6],
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Metal Pirates Room".to_string(),
                    },
                ),
                (
                    vec![0x56, 0xB6],
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Three Musketeers' Room".to_string(),
                    },
                ),
                (
                    vec![0x98, 0xB6],
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Ridley Tank Room".to_string(),
                    },
                ),
                (
                    vec![0xC1, 0xB6],
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Screw Attack Room".to_string(),
                    },
                ),
                (
                    vec![0xEE, 0xB6],
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Lower Norfair Fireflea Room".to_string(),
                    },
                ),
                (
                    vec![0x41, 0xB7],
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Red Kihunter Shaft Save Room".to_string(),
                    },
                ),
                (
                    vec![0x8E, 0xC9],
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Bowling Alley".to_string(),
                    },
                ),
                (
                    vec![0x08, 0xCA],
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Wrecked Ship Entrance".to_string(),
                    },
                ),
                (
                    vec![0x52, 0xCA],
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Attic".to_string(),
                    },
                ),
                (
                    vec![0xAE, 0xCA],
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Assembly Line".to_string(),
                    },
                ),
                (
                    vec![0xF6, 0xCA],
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Wrecked Ship Main Shaft".to_string(),
                    },
                ),
                (
                    vec![0x8B, 0xCB],
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Spiky Death Room".to_string(),
                    },
                ),
                (
                    vec![0xD5, 0xCB],
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Electric Death Room".to_string(),
                    },
                ),
                (
                    vec![0x27, 0xCC],
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Wrecked Ship Energy Tank Room".to_string(),
                    },
                ),
                (
                    vec![0x6F, 0xCC],
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Basement".to_string(),
                    },
                ),
                (
                    vec![0xCB, 0xCC],
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Wrecked Ship Map Room".to_string(),
                    },
                ),
                (
                    vec![0x13, 0xCD],
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Phantoon's Room".to_string(),
                    },
                ),
                (
                    vec![0x5C, 0xCD],
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Sponge Bath".to_string(),
                    },
                ),
                (
                    vec![0xA8, 0xCD],
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Wrecked Ship West Super Room".to_string(),
                    },
                ),
                (
                    vec![0xF1, 0xCD],
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Wrecked Ship East Super Room".to_string(),
                    },
                ),
                (
                    vec![0x40, 0xCE],
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Gravity Suit Room".to_string(),
                    },
                ),
                (
                    vec![0x8A, 0xCE],
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Wrecked Ship Save Room".to_string(),
                    },
                ),
                (
                    vec![0xD2, 0xCE],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Glass Tunnel Save Room".to_string(),
                    },
                ),
                (
                    vec![0xFB, 0xCE],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Glass Tunnel".to_string(),
                    },
                ),
                (
                    vec![0x54, 0xCF],
                    Location {
                        region: "Maridia".to_string(),
                        name: "West Glass Tube Tunnel".to_string(),
                    },
                ),
                (
                    vec![0x80, 0xCF],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Boyon Gate Hall".to_string(),
                    },
                ),
                (
                    vec![0xC9, 0xCF],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Main Street".to_string(),
                    },
                ),
                (
                    vec![0x17, 0xD0],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Fish Tank".to_string(),
                    },
                ),
                (
                    vec![0x55, 0xD0],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Mama Turtle Room".to_string(),
                    },
                ),
                (
                    vec![0x8A, 0xD0],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Crab Tunnel".to_string(),
                    },
                ),
                (
                    vec![0xB9, 0xD0],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Mt. Everest".to_string(),
                    },
                ),
                (
                    vec![0x04, 0xD1],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Red Fish Room".to_string(),
                    },
                ),
                (
                    vec![0x3B, 0xD1],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Watering Hole".to_string(),
                    },
                ),
                (
                    vec![0x6D, 0xD1],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Northwest Maridia Bug Room".to_string(),
                    },
                ),
                (
                    vec![0xA3, 0xD1],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Crab Shaft".to_string(),
                    },
                ),
                (
                    vec![0xDD, 0xD1],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Pseudo Plasma Spark Room".to_string(),
                    },
                ),
                (
                    vec![0x1C, 0xD2],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Crab Hole".to_string(),
                    },
                ),
                (
                    vec![0x52, 0xD2],
                    Location {
                        region: "Maridia".to_string(),
                        name: "West Sand Hall Tunnel".to_string(),
                    },
                ),
                (
                    vec![0x7E, 0xD2],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Plasma Tutorial Room".to_string(),
                    },
                ),
                (
                    vec![0xAA, 0xD2],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Plasma Room".to_string(),
                    },
                ),
                (
                    vec![0xD9, 0xD2],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Thread The Needle Room".to_string(),
                    },
                ),
                (
                    vec![0x0B, 0xD3],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Maridia Elevator Room".to_string(),
                    },
                ),
                (
                    vec![0x40, 0xD3],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Plasma Spark Room".to_string(),
                    },
                ),
                (
                    vec![0x87, 0xD3],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Kassiuz Room".to_string(),
                    },
                ),
                (
                    vec![0xB6, 0xD3],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Maridia Map Room".to_string(),
                    },
                ),
                (
                    vec![0xDF, 0xD3],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Forgotten Highway Save Room".to_string(),
                    },
                ),
                (
                    vec![0x08, 0xD4],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Toilet Bowl".to_string(),
                    },
                ),
                (
                    vec![0x33, 0xD4],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Bug Sand Hole".to_string(),
                    },
                ),
                (
                    vec![0x61, 0xD4],
                    Location {
                        region: "Maridia".to_string(),
                        name: "West Sand Hall".to_string(),
                    },
                ),
                (
                    vec![0x8E, 0xD4],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Oasis".to_string(),
                    },
                ),
                (
                    vec![0xC2, 0xD4],
                    Location {
                        region: "Maridia".to_string(),
                        name: "East Sand Hall".to_string(),
                    },
                ),
                (
                    vec![0xEF, 0xD4],
                    Location {
                        region: "Maridia".to_string(),
                        name: "West Sand Hole".to_string(),
                    },
                ),
                (
                    vec![0x1E, 0xD5],
                    Location {
                        region: "Maridia".to_string(),
                        name: "East Sand Hole".to_string(),
                    },
                ),
                (
                    vec![0x4D, 0xD5],
                    Location {
                        region: "Maridia".to_string(),
                        name: "West Aqueduct Quicksand Room".to_string(),
                    },
                ),
                (
                    vec![0x7A, 0xD5],
                    Location {
                        region: "Maridia".to_string(),
                        name: "East Aqueduct Quicksand Room".to_string(),
                    },
                ),
                (
                    vec![0xA7, 0xD5],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Aqueduct".to_string(),
                    },
                ),
                (
                    vec![0xEC, 0xD5],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Butterfly Room".to_string(),
                    },
                ),
                (
                    vec![0x17, 0xD6],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Botwoon Hallway".to_string(),
                    },
                ),
                (
                    vec![0x46, 0xD6],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Pants Room".to_string(),
                    },
                ),
                (
                    vec![0x9A, 0xD6],
                    Location {
                        region: "Maridia".to_string(),
                        name: "East Pants Room".to_string(),
                    },
                ),
                (
                    vec![0xD0, 0xD6],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Spring Ball Room".to_string(),
                    },
                ),
                (
                    vec![0xFD, 0xD6],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Below Botwoon Energy Tank".to_string(),
                    },
                ),
                (
                    vec![0x2A, 0xD7],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Colosseum".to_string(),
                    },
                ),
                (
                    vec![0x65, 0xD7],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Aqueduct Save Room".to_string(),
                    },
                ),
                (
                    vec![0x8F, 0xD7],
                    Location {
                        region: "Maridia".to_string(),
                        name: "The Precious Room".to_string(),
                    },
                ),
                (
                    vec![0xE4, 0xD7],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Botwoon Energy Tank Room".to_string(),
                    },
                ),
                (
                    vec![0x1A, 0xD8],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Draygon Save Room".to_string(),
                    },
                ),
                (
                    vec![0x45, 0xD8],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Maridia Missile Refill Room".to_string(),
                    },
                ),
                (
                    vec![0x6E, 0xD8],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Plasma Beach Quicksand Room".to_string(),
                    },
                ),
                (
                    vec![0x98, 0xD8],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Botwoon Quicksand Room".to_string(),
                    },
                ),
                (
                    vec![0xC5, 0xD8],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Shaktool Room".to_string(),
                    },
                ),
                (
                    vec![0x13, 0xD9],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Halfie Climb Room".to_string(),
                    },
                ),
                (
                    vec![0x5E, 0xD9],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Botwoon's Room".to_string(),
                    },
                ),
                (
                    vec![0xAA, 0xD9],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Space Jump Room".to_string(),
                    },
                ),
                (
                    vec![0xD4, 0xD9],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Maridia Health Refill Room".to_string(),
                    },
                ),
                (
                    vec![0xFE, 0xD9],
                    Location {
                        region: "Maridia".to_string(),
                        name: "West Cactus Alley Room".to_string(),
                    },
                ),
                (
                    vec![0x2B, 0xDA],
                    Location {
                        region: "Maridia".to_string(),
                        name: "East Cactus Alley Room".to_string(),
                    },
                ),
                (
                    vec![0x60, 0xDA],
                    Location {
                        region: "Maridia".to_string(),
                        name: "Draygon's Room".to_string(),
                    },
                ),
                (
                    vec![0xAE, 0xDA],
                    Location {
                        region: "Tourian".to_string(),
                        name: "Tourian First Room".to_string(),
                    },
                ),
                (
                    vec![0xE1, 0xDA],
                    Location {
                        region: "Tourian".to_string(),
                        name: "Metroid Room 1".to_string(),
                    },
                ),
                (
                    vec![0x31, 0xDB],
                    Location {
                        region: "Tourian".to_string(),
                        name: "Metroid Room 2".to_string(),
                    },
                ),
                (
                    vec![0x7D, 0xDB],
                    Location {
                        region: "Tourian".to_string(),
                        name: "Metroid Room 3".to_string(),
                    },
                ),
                (
                    vec![0xCD, 0xDB],
                    Location {
                        region: "Tourian".to_string(),
                        name: "Metroid Room 4".to_string(),
                    },
                ),
                (
                    vec![0x19, 0xDC],
                    Location {
                        region: "Tourian".to_string(),
                        name: "Blue Hopper Room".to_string(),
                    },
                ),
                (
                    vec![0x65, 0xDC],
                    Location {
                        region: "Tourian".to_string(),
                        name: "Dust Torizo Room".to_string(),
                    },
                ),
                (
                    vec![0xB1, 0xDC],
                    Location {
                        region: "Tourian".to_string(),
                        name: "Big Boy Room".to_string(),
                    },
                ),
                (
                    vec![0xFF, 0xDC],
                    Location {
                        region: "Tourian".to_string(),
                        name: "Seaweed Room".to_string(),
                    },
                ),
                (
                    vec![0x2E, 0xDD],
                    Location {
                        region: "Tourian".to_string(),
                        name: "Tourian Recharge Room".to_string(),
                    },
                ),
                (
                    vec![0x58, 0xDD],
                    Location {
                        region: "Tourian".to_string(),
                        name: "Mother Brain Room".to_string(),
                    },
                ),
                (
                    vec![0xC4, 0xDD],
                    Location {
                        region: "Tourian".to_string(),
                        name: "Tourian Eye Door Room".to_string(),
                    },
                ),
                (
                    vec![0xF3, 0xDD],
                    Location {
                        region: "Tourian".to_string(),
                        name: "Rinka Shaft".to_string(),
                    },
                ),
                (
                    vec![0x23, 0xDE],
                    Location {
                        region: "Tourian".to_string(),
                        name: "Lower Tourian Save Room".to_string(),
                    },
                ),
                (
                    vec![0x4D, 0xDE],
                    Location {
                        region: "Tourian".to_string(),
                        name: "Tourian Escape Room 1".to_string(),
                    },
                ),
                (
                    vec![0x7A, 0xDE],
                    Location {
                        region: "Tourian".to_string(),
                        name: "Tourian Escape Room 2".to_string(),
                    },
                ),
                (
                    vec![0xA7, 0xDE],
                    Location {
                        region: "Tourian".to_string(),
                        name: "Tourian Escape Room 3".to_string(),
                    },
                ),
                (
                    vec![0xDE, 0xDE],
                    Location {
                        region: "Tourian".to_string(),
                        name: "Tourian Escape Room 4".to_string(),
                    },
                ),
                (
                    vec![0x1B, 0xDF],
                    Location {
                        region: "Tourian".to_string(),
                        name: "Upper Tourian Save Room".to_string(),
                    },
                ),
                (
                    vec![0x45, 0xDF],
                    Location {
                        region: "Ceres".to_string(),
                        name: "Ceres Elevator Room".to_string(),
                    },
                ),
                (
                    vec![0x8D, 0xDF],
                    Location {
                        region: "Ceres".to_string(),
                        name: "Falling Tile Room".to_string(),
                    },
                ),
                (
                    vec![0xD7, 0xDF],
                    Location {
                        region: "Ceres".to_string(),
                        name: "Magnet Stairs Room".to_string(),
                    },
                ),
                (
                    vec![0x21, 0xE0],
                    Location {
                        region: "Ceres".to_string(),
                        name: "Dead Scientist Room".to_string(),
                    },
                ),
                (
                    vec![0x6B, 0xE0],
                    Location {
                        region: "Ceres".to_string(),
                        name: "58 Escape".to_string(),
                    },
                ),
                (
                    vec![0xB5, 0xE0],
                    Location {
                        region: "Ceres".to_string(),
                        name: "Ceres Ridley's Room".to_string(),
                    },
                ),
                (
                    vec![0x2C, 0xE8],
                    Location {
                        region: "UNUSED".to_string(),
                        name: "Debug room".to_string(),
                    },
                ),
            ]);

            rooms_data.get(&id).unwrap_or(&Location::empty()).to_owned()
        }
    }
}
