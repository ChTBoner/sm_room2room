pub mod room_data {
    use crate::supermetroid::super_metroid::GameTime;
    use std::collections::HashMap;
    use time::Duration;
    // use csv;
    // use std::fs::File;
    // use std::path::Path;

    #[derive(Debug, PartialEq, Clone)]
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

    #[derive(Debug, PartialEq, Clone)]
    pub struct Room {
        pub id: String,
        pub location: Location,
        pub rta_entry: Duration,
        pub igt_entry: GameTime,
    }

    impl Room {
        pub fn new(id: String, rta: Duration, igt: GameTime) -> Self {
            Self {
                id: id.to_owned(),
                location: Self::get_room_data(id),
                rta_entry: rta,
                igt_entry: igt,
            }
        }

        fn get_room_data(id: String) -> Location {
            let rooms_data: HashMap<String, Location> = HashMap::from([
                (
                    "0x91F8".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Landing Site".to_string(),
                    },
                ),
                (
                    "0x92B3".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Gauntlet Entrance".to_string(),
                    },
                ),
                (
                    "0x92FD".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Parlor and Alcatraz".to_string(),
                    },
                ),
                (
                    "0x93AA".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Crateria Power Bomb Room".to_string(),
                    },
                ),
                (
                    "0x93D5".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Crateria Save Room".to_string(),
                    },
                ),
                (
                    "0x93FE".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "West Ocean".to_string(),
                    },
                ),
                (
                    "0x9461".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Bowling Alley Path".to_string(),
                    },
                ),
                (
                    "0x948C".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Crateria Kihunter Room".to_string(),
                    },
                ),
                (
                    "0x94CC".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Forgotten Highway Elevator".to_string(),
                    },
                ),
                (
                    "0x94FD".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "East Ocean".to_string(),
                    },
                ),
                (
                    "0x9552".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Forgotten Highway Kago Room".to_string(),
                    },
                ),
                (
                    "0x957D".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Crab Maze".to_string(),
                    },
                ),
                (
                    "0x95A8".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Forgotten Highway Elbow".to_string(),
                    },
                ),
                (
                    "0x95D4".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Crateria Tube".to_string(),
                    },
                ),
                (
                    "0x95FF".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "The Moat".to_string(),
                    },
                ),
                (
                    "0x962A".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Red Brinstar Elevator Room".to_string(),
                    },
                ),
                (
                    "0x965B".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Gauntlet Energy Tank Room".to_string(),
                    },
                ),
                (
                    "0x968F".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "West Ocean Bridge".to_string(),
                    },
                ),
                (
                    "0x96BA".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Climb".to_string(),
                    },
                ),
                (
                    "0x975C".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Pit Room".to_string(),
                    },
                ),
                (
                    "0x97B5".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Blue Brinstar Elevator Room".to_string(),
                    },
                ),
                (
                    "0x9804".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Bomb Torizo Room".to_string(),
                    },
                ),
                (
                    "0x9879".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Flyway".to_string(),
                    },
                ),
                (
                    "0x98E2".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Pre-Map Flyway".to_string(),
                    },
                ),
                (
                    "0x990D".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Terminator Room".to_string(),
                    },
                ),
                (
                    "0x9938".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Green Brinstar Elevator Room".to_string(),
                    },
                ),
                (
                    "0x9969".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Lower Mushrooms".to_string(),
                    },
                ),
                (
                    "0x9994".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Crateria Map Room".to_string(),
                    },
                ),
                (
                    "0x99BD".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Green Pirates Shaft".to_string(),
                    },
                ),
                (
                    "0x99F9".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Crateria Super Room".to_string(),
                    },
                ),
                (
                    "0x9A44".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Final Missile Bombway".to_string(),
                    },
                ),
                (
                    "0x9A90".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "The Final Missile".to_string(),
                    },
                ),
                (
                    "0x9AD9".to_string(),
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Green Brinstar Main Shaft".to_string(),
                    },
                ),
                (
                    "0x9B5B".to_string(),
                    Location {
                        region: "Pink Brinstar".to_string(),
                        name: "Spore Spawn Super Room".to_string(),
                    },
                ),
                (
                    "0x9B9D".to_string(),
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Brinstar Pre-Map Room".to_string(),
                    },
                ),
                (
                    "0x9BC8".to_string(),
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Early Supers Room".to_string(),
                    },
                ),
                (
                    "0x9C07".to_string(),
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Brinstar Reserve Tank Room".to_string(),
                    },
                ),
                (
                    "0x9C35".to_string(),
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Brinstar Map Room".to_string(),
                    },
                ),
                (
                    "0x9C5E".to_string(),
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Green Brinstar Fireflea Room".to_string(),
                    },
                ),
                (
                    "0x9C89".to_string(),
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Green Brinstar Missile Station".to_string(),
                    },
                ),
                (
                    "0x9CB3".to_string(),
                    Location {
                        region: "Pink Brinstar".to_string(),
                        name: "Dachora Room".to_string(),
                    },
                ),
                (
                    "0x9D19".to_string(),
                    Location {
                        region: "Pink Brinstar".to_string(),
                        name: "Big Pink".to_string(),
                    },
                ),
                (
                    "0x9D9C".to_string(),
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Spore Spawn Kihunter Room".to_string(),
                    },
                ),
                (
                    "0x9DC7".to_string(),
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Spore Spawn Room".to_string(),
                    },
                ),
                (
                    "0x9E11".to_string(),
                    Location {
                        region: "Pink Brinstar".to_string(),
                        name: "Pink Brinstar Power Bomb Room".to_string(),
                    },
                ),
                (
                    "0x9E52".to_string(),
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Green Hill Zone".to_string(),
                    },
                ),
                (
                    "0x9E9F".to_string(),
                    Location {
                        region: "Blue Brinstar".to_string(),
                        name: "Morph Ball Room".to_string(),
                    },
                ),
                (
                    "0x9F11".to_string(),
                    Location {
                        region: "Blue Brinstar".to_string(),
                        name: "Construction Zone".to_string(),
                    },
                ),
                (
                    "0x9F64".to_string(),
                    Location {
                        region: "Blue Brinstar".to_string(),
                        name: "Blue Brinstar Energy Tank Room".to_string(),
                    },
                ),
                (
                    "0x9FBA".to_string(),
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Noob Bridge".to_string(),
                    },
                ),
                (
                    "0x9FE5".to_string(),
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Green Brinstar Beetom Room".to_string(),
                    },
                ),
                (
                    "0xA011".to_string(),
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Etecoon Energy Tank Room".to_string(),
                    },
                ),
                (
                    "0xA051".to_string(),
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Etecoon Super Room".to_string(),
                    },
                ),
                (
                    "0xA07B".to_string(),
                    Location {
                        region: "Pink Brinstar".to_string(),
                        name: "Dachora Energy Charge Station".to_string(),
                    },
                ),
                (
                    "0xA0A4".to_string(),
                    Location {
                        region: "Pink Brinstar".to_string(),
                        name: "Spore Spawn Farming Room".to_string(),
                    },
                ),
                (
                    "0xA0D2".to_string(),
                    Location {
                        region: "Pink Brinstar".to_string(),
                        name: "Waterway Energy Tank Room".to_string(),
                    },
                ),
                (
                    "0xA107".to_string(),
                    Location {
                        region: "Blue Brinstar".to_string(),
                        name: "First Missile Room".to_string(),
                    },
                ),
                (
                    "0xA130".to_string(),
                    Location {
                        region: "Pink Brinstar".to_string(),
                        name: "Pink Brinstar Hopper Room".to_string(),
                    },
                ),
                (
                    "0xA15B".to_string(),
                    Location {
                        region: "Pink Brinstar".to_string(),
                        name: "Hopper Energy Tank Room".to_string(),
                    },
                ),
                (
                    "0xA184".to_string(),
                    Location {
                        region: "Pink Brinstar".to_string(),
                        name: "Big Pink Save Room".to_string(),
                    },
                ),
                (
                    "0xA1AD".to_string(),
                    Location {
                        region: "Blue Brinstar".to_string(),
                        name: "Blue Brinstar Boulder Room".to_string(),
                    },
                ),
                (
                    "0xA1D8".to_string(),
                    Location {
                        region: "Blue Brinstar".to_string(),
                        name: "Billy Mays Room".to_string(),
                    },
                ),
                (
                    "0xA201".to_string(),
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Green Brinstar Save Room".to_string(),
                    },
                ),
                (
                    "0xA22A".to_string(),
                    Location {
                        region: "Green Brinstar".to_string(),
                        name: "Etecoon Save Room".to_string(),
                    },
                ),
                (
                    "0xA253".to_string(),
                    Location {
                        region: "Red Brinstar".to_string(),
                        name: "Red Tower".to_string(),
                    },
                ),
                (
                    "0xA293".to_string(),
                    Location {
                        region: "Red Brinstar".to_string(),
                        name: "Red Brinstar Fireflea Room".to_string(),
                    },
                ),
                (
                    "0xA2CE".to_string(),
                    Location {
                        region: "Red Brinstar".to_string(),
                        name: "X-Ray Scope Room".to_string(),
                    },
                ),
                (
                    "0xA2F7".to_string(),
                    Location {
                        region: "Red Brinstar".to_string(),
                        name: "Hellway".to_string(),
                    },
                ),
                (
                    "0xA322".to_string(),
                    Location {
                        region: "Red Brinstar".to_string(),
                        name: "Caterpillar Room".to_string(),
                    },
                ),
                (
                    "0xA37C".to_string(),
                    Location {
                        region: "Red Brinstar".to_string(),
                        name: "Beta Power Bomb Room".to_string(),
                    },
                ),
                (
                    "0xA3AE".to_string(),
                    Location {
                        region: "Red Brinstar".to_string(),
                        name: "Alpha Power Bomb Room".to_string(),
                    },
                ),
                (
                    "0xA3DD".to_string(),
                    Location {
                        region: "Red Brinstar".to_string(),
                        name: "Bat Room".to_string(),
                    },
                ),
                (
                    "0xA408".to_string(),
                    Location {
                        region: "Red Brinstar".to_string(),
                        name: "Below Spazer".to_string(),
                    },
                ),
                (
                    "0xA447".to_string(),
                    Location {
                        region: "Red Brinstar".to_string(),
                        name: "Spazer Room".to_string(),
                    },
                ),
                (
                    "0xA471".to_string(),
                    Location {
                        region: "Warehouse".to_string(),
                        name: "Warehouse Zeela Room".to_string(),
                    },
                ),
                (
                    "0xA4B1".to_string(),
                    Location {
                        region: "Warehouse".to_string(),
                        name: "Warehouse Energy Tank Room".to_string(),
                    },
                ),
                (
                    "0xA4DA".to_string(),
                    Location {
                        region: "Warehouse".to_string(),
                        name: "Warehouse Kihunter Room".to_string(),
                    },
                ),
                (
                    "0xA521".to_string(),
                    Location {
                        region: "Warehouse".to_string(),
                        name: "Baby Kraid Room".to_string(),
                    },
                ),
                (
                    "0xA56B".to_string(),
                    Location {
                        region: "Warehouse".to_string(),
                        name: "Kraid Eye Door Room".to_string(),
                    },
                ),
                (
                    "0xA59F".to_string(),
                    Location {
                        region: "Warehouse".to_string(),
                        name: "Kraid Room".to_string(),
                    },
                ),
                (
                    "0xA5ED".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Statues Hallway".to_string(),
                    },
                ),
                (
                    "0xA618".to_string(),
                    Location {
                        region: "Red Brinstar".to_string(),
                        name: "Sloaters Refill".to_string(),
                    },
                ),
                (
                    "0xA641".to_string(),
                    Location {
                        region: "Warehouse".to_string(),
                        name: "Kraid Recharge Stations".to_string(),
                    },
                ),
                (
                    "0xA66A".to_string(),
                    Location {
                        region: "Crateria".to_string(),
                        name: "Statues Room".to_string(),
                    },
                ),
                (
                    "0xA6A1".to_string(),
                    Location {
                        region: "Warehouse".to_string(),
                        name: "Warehouse Entrance".to_string(),
                    },
                ),
                (
                    "0xA6E2".to_string(),
                    Location {
                        region: "Warehouse".to_string(),
                        name: "Varia Suit Room".to_string(),
                    },
                ),
                (
                    "0xA70B".to_string(),
                    Location {
                        region: "Warehouse".to_string(),
                        name: "Warehouse Save Room".to_string(),
                    },
                ),
                (
                    "0xA734".to_string(),
                    Location {
                        region: "Red Brinstar".to_string(),
                        name: "Red Brinstar Save Room".to_string(),
                    },
                ),
                (
                    "0xA75D".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Ice Beam Acid Room".to_string(),
                    },
                ),
                (
                    "0xA788".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Cathedral".to_string(),
                    },
                ),
                (
                    "0xA7B3".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Cathedral Entrance".to_string(),
                    },
                ),
                (
                    "0xA7DE".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Business Center".to_string(),
                    },
                ),
                (
                    "0xA815".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Ice Beam Gate Room".to_string(),
                    },
                ),
                (
                    "0xA865".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Ice Beam Tutorial Room".to_string(),
                    },
                ),
                (
                    "0xA890".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Ice Beam Room".to_string(),
                    },
                ),
                (
                    "0xA8B9".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Ice Beam Snake Room".to_string(),
                    },
                ),
                (
                    "0xA8F8".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Crumble Shaft".to_string(),
                    },
                ),
                (
                    "0xA923".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Crocomire Speedway".to_string(),
                    },
                ),
                (
                    "0xA98D".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Crocomire's Room".to_string(),
                    },
                ),
                (
                    "0xA9E5".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Hi Jump Boots Room".to_string(),
                    },
                ),
                (
                    "0xAA0E".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Crocomire Escape".to_string(),
                    },
                ),
                (
                    "0xAA41".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Hi Jump Energy Tank Room".to_string(),
                    },
                ),
                (
                    "0xAA82".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Post Crocomire Farming Room".to_string(),
                    },
                ),
                (
                    "0xAAB5".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Post Crocomire Save Room".to_string(),
                    },
                ),
                (
                    "0xAADE".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Post Crocomire Power Bomb Room".to_string(),
                    },
                ),
                (
                    "0xAB07".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Post Crocomire Shaft".to_string(),
                    },
                ),
                (
                    "0xAB3B".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Post Crocomire Missile Room".to_string(),
                    },
                ),
                (
                    "0xAB64".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Grapple Tutorial Room 3".to_string(),
                    },
                ),
                (
                    "0xAB8F".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Post Crocomire Jump Room".to_string(),
                    },
                ),
                (
                    "0xABD2".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Grapple Tutorial Room 2".to_string(),
                    },
                ),
                (
                    "0xAC00".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Grapple Tutorial Room 1".to_string(),
                    },
                ),
                (
                    "0xAC2B".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Grapple Beam Room".to_string(),
                    },
                ),
                (
                    "0xAC5A".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Norfair Reserve Tank Room".to_string(),
                    },
                ),
                (
                    "0xAC83".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Green Bubbles Missile Room".to_string(),
                    },
                ),
                (
                    "0xACB3".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Bubble Mountain".to_string(),
                    },
                ),
                (
                    "0xACF0".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Speed Booster Hall".to_string(),
                    },
                ),
                (
                    "0xAD1B".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Speed Booster Room".to_string(),
                    },
                ),
                (
                    "0xAD5E".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Single Chamber".to_string(),
                    },
                ),
                (
                    "0xADAD".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Double Chamber".to_string(),
                    },
                ),
                (
                    "0xADDE".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Wave Beam Room".to_string(),
                    },
                ),
                (
                    "0xAE07".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Spiky Platforms Tunnel".to_string(),
                    },
                ),
                (
                    "0xAE32".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Volcano Room".to_string(),
                    },
                ),
                (
                    "0xAE74".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Kronic Boost Room".to_string(),
                    },
                ),
                (
                    "0xAEB4".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Magdollite Tunnel".to_string(),
                    },
                ),
                (
                    "0xAEDF".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Purple Shaft".to_string(),
                    },
                ),
                (
                    "0xAF14".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Lava Dive Room".to_string(),
                    },
                ),
                (
                    "0xAF3F".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Lower Norfair Elevator".to_string(),
                    },
                ),
                (
                    "0xAF72".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Upper Norfair Farming Room".to_string(),
                    },
                ),
                (
                    "0xAFA3".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Rising Tide".to_string(),
                    },
                ),
                (
                    "0xAFCE".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Acid Snakes Tunnel".to_string(),
                    },
                ),
                (
                    "0xAFFB".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Spiky Acid Snakes Tunnel".to_string(),
                    },
                ),
                (
                    "0xB026".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Nutella Refill".to_string(),
                    },
                ),
                (
                    "0xB051".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Purple Farming Room".to_string(),
                    },
                ),
                (
                    "0xB07A".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Bat Cave".to_string(),
                    },
                ),
                (
                    "0xB0B4".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Norfair Map Room".to_string(),
                    },
                ),
                (
                    "0xB0DD".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Bubble Mountain Save Room".to_string(),
                    },
                ),
                (
                    "0xB106".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Frog Speedway".to_string(),
                    },
                ),
                (
                    "0xB139".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Red Pirate Shaft".to_string(),
                    },
                ),
                (
                    "0xB167".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Frog Savestation".to_string(),
                    },
                ),
                (
                    "0xB192".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Crocomire Save Room".to_string(),
                    },
                ),
                (
                    "0xB1BB".to_string(),
                    Location {
                        region: "Upper Norfair".to_string(),
                        name: "Lower Norfair Elevator Save Room".to_string(),
                    },
                ),
                (
                    "0xB1E5".to_string(),
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Acid Statue Room".to_string(),
                    },
                ),
                (
                    "0xB236".to_string(),
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Main Hall".to_string(),
                    },
                ),
                (
                    "0xB283".to_string(),
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Golden Torizo's Room".to_string(),
                    },
                ),
                (
                    "0xB2DA".to_string(),
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Fast Ripper Room".to_string(),
                    },
                ),
                (
                    "0xB305".to_string(),
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Golden Torizo Energy Recharge".to_string(),
                    },
                ),
                (
                    "0xB32E".to_string(),
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Ridley's Room".to_string(),
                    },
                ),
                (
                    "0xB37A".to_string(),
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Lower Norfair Farming Room".to_string(),
                    },
                ),
                (
                    "0xB3A5".to_string(),
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Fast Pillars Setup Room".to_string(),
                    },
                ),
                (
                    "0xB3E1".to_string(),
                    Location {
                        region: "".to_string(),
                        name: "Unused Crocomire Room".to_string(),
                    },
                ),
                (
                    "0xB40A".to_string(),
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Mickey Mouse Room".to_string(),
                    },
                ),
                (
                    "0xB457".to_string(),
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Pillar Room".to_string(),
                    },
                ),
                (
                    "0xB482".to_string(),
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Plowerhouse Room".to_string(),
                    },
                ),
                (
                    "0xB4AD".to_string(),
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "The Worst Room In The Game".to_string(),
                    },
                ),
                (
                    "0xB4E5".to_string(),
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Amphitheatre".to_string(),
                    },
                ),
                (
                    "0xB510".to_string(),
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Lower Norfair Spring Ball Maze Room".to_string(),
                    },
                ),
                (
                    "0xB55A".to_string(),
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Lower Norfair Escape Power Bomb Room".to_string(),
                    },
                ),
                (
                    "0xB585".to_string(),
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Red Kihunter Shaft".to_string(),
                    },
                ),
                (
                    "0xB5D5".to_string(),
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Wasteland".to_string(),
                    },
                ),
                (
                    "0xB62B".to_string(),
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Metal Pirates Room".to_string(),
                    },
                ),
                (
                    "0xB656".to_string(),
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Three Musketeers' Room".to_string(),
                    },
                ),
                (
                    "0xB698".to_string(),
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Ridley Tank Room".to_string(),
                    },
                ),
                (
                    "0xB6C1".to_string(),
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Screw Attack Room".to_string(),
                    },
                ),
                (
                    "0xB6EE".to_string(),
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Lower Norfair Fireflea Room".to_string(),
                    },
                ),
                (
                    "0xB741".to_string(),
                    Location {
                        region: "Lower Norfair".to_string(),
                        name: "Red Kihunter Shaft Save Room".to_string(),
                    },
                ),
                (
                    "0xC98E".to_string(),
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Bowling Alley".to_string(),
                    },
                ),
                (
                    "0xCA08".to_string(),
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Wrecked Ship Entrance".to_string(),
                    },
                ),
                (
                    "0xCA52".to_string(),
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Attic".to_string(),
                    },
                ),
                (
                    "0xCAAE".to_string(),
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Assembly Line".to_string(),
                    },
                ),
                (
                    "0xCAF6".to_string(),
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Wrecked Ship Main Shaft".to_string(),
                    },
                ),
                (
                    "0xCB8B".to_string(),
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Spiky Death Room".to_string(),
                    },
                ),
                (
                    "0xCBD5".to_string(),
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Electric Death Room".to_string(),
                    },
                ),
                (
                    "0xCC27".to_string(),
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Wrecked Ship Energy Tank Room".to_string(),
                    },
                ),
                (
                    "0xCC6F".to_string(),
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Basement".to_string(),
                    },
                ),
                (
                    "0xCCCB".to_string(),
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Wrecked Ship Map Room".to_string(),
                    },
                ),
                (
                    "0xCD13".to_string(),
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Phantoon's Room".to_string(),
                    },
                ),
                (
                    "0xCD5C".to_string(),
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Sponge Bath".to_string(),
                    },
                ),
                (
                    "0xCDA8".to_string(),
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Wrecked Ship West Super Room".to_string(),
                    },
                ),
                (
                    "0xCDF1".to_string(),
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Wrecked Ship East Super Room".to_string(),
                    },
                ),
                (
                    "0xCE40".to_string(),
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Gravity Suit Room".to_string(),
                    },
                ),
                (
                    "0xCE8A".to_string(),
                    Location {
                        region: "Wrecked Ship".to_string(),
                        name: "Wrecked Ship Save Room".to_string(),
                    },
                ),
                (
                    "0xCED2".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Glass Tunnel Save Room".to_string(),
                    },
                ),
                (
                    "0xCEFB".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Glass Tunnel".to_string(),
                    },
                ),
                (
                    "0xCF54".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "West Glass Tube Tunnel".to_string(),
                    },
                ),
                (
                    "0xCF80".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Boyon Gate Hall".to_string(),
                    },
                ),
                (
                    "0xCFC9".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Main Street".to_string(),
                    },
                ),
                (
                    "0xD017".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Fish Tank".to_string(),
                    },
                ),
                (
                    "0xD055".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Mama Turtle Room".to_string(),
                    },
                ),
                (
                    "0xD08A".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Crab Tunnel".to_string(),
                    },
                ),
                (
                    "0xD0B9".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Mt. Everest".to_string(),
                    },
                ),
                (
                    "0xD104".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Red Fish Room".to_string(),
                    },
                ),
                (
                    "0xD13B".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Watering Hole".to_string(),
                    },
                ),
                (
                    "0xD16D".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Northwest Maridia Bug Room".to_string(),
                    },
                ),
                (
                    "0xD1A3".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Crab Shaft".to_string(),
                    },
                ),
                (
                    "0xD1DD".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Pseudo Plasma Spark Room".to_string(),
                    },
                ),
                (
                    "0xD21C".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Crab Hole".to_string(),
                    },
                ),
                (
                    "0xD252".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "West Sand Hall Tunnel".to_string(),
                    },
                ),
                (
                    "0xD27E".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Plasma Tutorial Room".to_string(),
                    },
                ),
                (
                    "0xD2AA".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Plasma Room".to_string(),
                    },
                ),
                (
                    "0xD2D9".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Thread The Needle Room".to_string(),
                    },
                ),
                (
                    "0xD30B".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Maridia Elevator Room".to_string(),
                    },
                ),
                (
                    "0xD340".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Plasma Spark Room".to_string(),
                    },
                ),
                (
                    "0xD387".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Kassiuz Room".to_string(),
                    },
                ),
                (
                    "0xD3B6".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Maridia Map Room".to_string(),
                    },
                ),
                (
                    "0xD3DF".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Forgotten Highway Save Room".to_string(),
                    },
                ),
                (
                    "0xD408".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Toilet Bowl".to_string(),
                    },
                ),
                (
                    "0xD433".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Bug Sand Hole".to_string(),
                    },
                ),
                (
                    "0xD461".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "West Sand Hall".to_string(),
                    },
                ),
                (
                    "0xD48E".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Oasis".to_string(),
                    },
                ),
                (
                    "0xD4C2".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "East Sand Hall".to_string(),
                    },
                ),
                (
                    "0xD4EF".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "West Sand Hole".to_string(),
                    },
                ),
                (
                    "0xD51E".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "East Sand Hole".to_string(),
                    },
                ),
                (
                    "0xD54D".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "West Aqueduct Quicksand Room".to_string(),
                    },
                ),
                (
                    "0xD57A".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "East Aqueduct Quicksand Room".to_string(),
                    },
                ),
                (
                    "0xD5A7".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Aqueduct".to_string(),
                    },
                ),
                (
                    "0xD5EC".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Butterfly Room".to_string(),
                    },
                ),
                (
                    "0xD617".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Botwoon Hallway".to_string(),
                    },
                ),
                (
                    "0xD646".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Pants Room".to_string(),
                    },
                ),
                (
                    "0xD69A".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "East Pants Room".to_string(),
                    },
                ),
                (
                    "0xD6D0".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Spring Ball Room".to_string(),
                    },
                ),
                (
                    "0xD6FD".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Below Botwoon Energy Tank".to_string(),
                    },
                ),
                (
                    "0xD72A".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Colosseum".to_string(),
                    },
                ),
                (
                    "0xD765".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Aqueduct Save Room".to_string(),
                    },
                ),
                (
                    "0xD78F".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "The Precious Room".to_string(),
                    },
                ),
                (
                    "0xD7E4".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Botwoon Energy Tank Room".to_string(),
                    },
                ),
                (
                    "0xD81A".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Draygon Save Room".to_string(),
                    },
                ),
                (
                    "0xD845".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Maridia Missile Refill Room".to_string(),
                    },
                ),
                (
                    "0xD86E".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Plasma Beach Quicksand Room".to_string(),
                    },
                ),
                (
                    "0xD898".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Botwoon Quicksand Room".to_string(),
                    },
                ),
                (
                    "0xD8C5".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Shaktool Room".to_string(),
                    },
                ),
                (
                    "0xD913".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Halfie Climb Room".to_string(),
                    },
                ),
                (
                    "0xD95E".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Botwoon's Room".to_string(),
                    },
                ),
                (
                    "0xD9AA".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Space Jump Room".to_string(),
                    },
                ),
                (
                    "0xD9D4".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Maridia Health Refill Room".to_string(),
                    },
                ),
                (
                    "0xD9FE".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "West Cactus Alley Room".to_string(),
                    },
                ),
                (
                    "0xDA2B".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "East Cactus Alley Room".to_string(),
                    },
                ),
                (
                    "0xDA60".to_string(),
                    Location {
                        region: "Maridia".to_string(),
                        name: "Draygon's Room".to_string(),
                    },
                ),
                (
                    "0xDAAE".to_string(),
                    Location {
                        region: "Tourian".to_string(),
                        name: "Tourian First Room".to_string(),
                    },
                ),
                (
                    "0xDAE1".to_string(),
                    Location {
                        region: "Tourian".to_string(),
                        name: "Metroid Room 1".to_string(),
                    },
                ),
                (
                    "0xDB31".to_string(),
                    Location {
                        region: "Tourian".to_string(),
                        name: "Metroid Room 2".to_string(),
                    },
                ),
                (
                    "0xDB7D".to_string(),
                    Location {
                        region: "Tourian".to_string(),
                        name: "Metroid Room 3".to_string(),
                    },
                ),
                (
                    "0xDBCD".to_string(),
                    Location {
                        region: "Tourian".to_string(),
                        name: "Metroid Room 4".to_string(),
                    },
                ),
                (
                    "0xDC19".to_string(),
                    Location {
                        region: "Tourian".to_string(),
                        name: "Blue Hopper Room".to_string(),
                    },
                ),
                (
                    "0xDC65".to_string(),
                    Location {
                        region: "Tourian".to_string(),
                        name: "Dust Torizo Room".to_string(),
                    },
                ),
                (
                    "0xDCB1".to_string(),
                    Location {
                        region: "Tourian".to_string(),
                        name: "Big Boy Room".to_string(),
                    },
                ),
                (
                    "0xDCFF".to_string(),
                    Location {
                        region: "Tourian".to_string(),
                        name: "Seaweed Room".to_string(),
                    },
                ),
                (
                    "0xDD2E".to_string(),
                    Location {
                        region: "Tourian".to_string(),
                        name: "Tourian Recharge Room".to_string(),
                    },
                ),
                (
                    "0xDD58".to_string(),
                    Location {
                        region: "Tourian".to_string(),
                        name: "Mother Brain Room".to_string(),
                    },
                ),
                (
                    "0xDDC4".to_string(),
                    Location {
                        region: "Tourian".to_string(),
                        name: "Tourian Eye Door Room".to_string(),
                    },
                ),
                (
                    "0xDDF3".to_string(),
                    Location {
                        region: "Tourian".to_string(),
                        name: "Rinka Shaft".to_string(),
                    },
                ),
                (
                    "0xDE23".to_string(),
                    Location {
                        region: "Tourian".to_string(),
                        name: "Lower Tourian Save Room".to_string(),
                    },
                ),
                (
                    "0xDE4D".to_string(),
                    Location {
                        region: "Tourian".to_string(),
                        name: "Tourian Escape Room 1".to_string(),
                    },
                ),
                (
                    "0xDE7A".to_string(),
                    Location {
                        region: "Tourian".to_string(),
                        name: "Tourian Escape Room 2".to_string(),
                    },
                ),
                (
                    "0xDEA7".to_string(),
                    Location {
                        region: "Tourian".to_string(),
                        name: "Tourian Escape Room 3".to_string(),
                    },
                ),
                (
                    "0xDEDE".to_string(),
                    Location {
                        region: "Tourian".to_string(),
                        name: "Tourian Escape Room 4".to_string(),
                    },
                ),
                (
                    "0xDF1B".to_string(),
                    Location {
                        region: "Tourian".to_string(),
                        name: "Upper Tourian Save Room".to_string(),
                    },
                ),
                (
                    "0xDF45".to_string(),
                    Location {
                        region: "Ceres".to_string(),
                        name: "Ceres Elevator Room".to_string(),
                    },
                ),
                (
                    "0xDF8D".to_string(),
                    Location {
                        region: "Ceres".to_string(),
                        name: "Falling Tile Room".to_string(),
                    },
                ),
                (
                    "0xDFD7".to_string(),
                    Location {
                        region: "Ceres".to_string(),
                        name: "Magnet Stairs Room".to_string(),
                    },
                ),
                (
                    "0xE021".to_string(),
                    Location {
                        region: "Ceres".to_string(),
                        name: "Dead Scientist Room".to_string(),
                    },
                ),
                (
                    "0xE06B".to_string(),
                    Location {
                        region: "Ceres".to_string(),
                        name: "58 Escape".to_string(),
                    },
                ),
                (
                    "0xE0B5".to_string(),
                    Location {
                        region: "Ceres".to_string(),
                        name: "Ceres Ridley's Room".to_string(),
                    },
                ),
                (
                    "0xE82C".to_string(),
                    Location {
                        region: "UNUSED".to_string(),
                        name: "Debug room".to_string(),
                    },
                ),
            ]);

            rooms_data.get(&id).unwrap_or(&Location::empty()).to_owned()
        }

        // fn parse_csv() -> csv::Reader<File> {
        //     let csv_path = Path::new("./data/super_metroid_rooms.csv");
        //     match File::open(csv_path) {
        //         Ok(csv_file) => csv::Reader::from_reader(csv_file),
        //         Err(e) => panic!("Error parsing csv file : {:?}", e),
        //     }
        // }

        // pub fn room_data_gen() -> HashMap<String, Room> {
        //     let mut csv_reader = parse_csv();

        //     let mut room_data_map: HashMap<String, Room> = HashMap::new();
        //     for result in csv_reader.records() {
        //         match result {
        //             Ok(record) => {
        //                 room_data_map.insert(
        //                     record[0].to_owned(),
        //                     Room {
        //                         smile_id: record[0].to_owned(),
        //                         region: record[1].to_owned(),
        //                         subregion: record[2].to_owned(),
        //                         room_name: record[3].to_owned(),
        //                     },
        //                 );
        //             }
        //             Err(e) => panic!("Error parsing csv file : {:?}", e),
        //         }
        //     }

        //     room_data_map
        // }
    }
}
