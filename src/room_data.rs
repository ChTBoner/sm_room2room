pub mod room_data {
    use csv;
    use std::collections::HashMap;
    use std::fs::File;
    use std::path::Path;

    #[derive(Debug, PartialEq, Clone)]
    pub struct Room {
        pub smile_id: String,
        pub region: String,
        pub subregion: String,
        pub room_name: String,
    }

    impl Room {
        pub fn new() -> Self {
            Self {
                smile_id: "".to_string(),
                region: "".to_string(),
                subregion: "".to_string(),
                room_name: "".to_string(),
            }
        }
    }

    fn parse_csv() -> csv::Reader<File> {
        let csv_path = Path::new("./data/super_metroid_rooms.csv");
        match File::open(csv_path) {
            Ok(csv_file) => return csv::Reader::from_reader(csv_file),
            Err(e) => panic!("Error parsing csv file : {:?}", e),
        }
    }

    pub fn room_data_gen() -> HashMap<String, Room> {
        let mut csv_reader = parse_csv();

        let mut room_data_map: HashMap<String, Room> = HashMap::new();
        for result in csv_reader.records() {
            match result {
                Ok(record) => {
                    room_data_map.insert(
                        record[0].to_owned(),
                        Room {
                            smile_id: record[0].to_owned(),
                            region: record[1].to_owned(),
                            subregion: record[2].to_owned(),
                            room_name: record[3].to_owned(),
                        }
                    );
                }
                Err(e) => panic!("Error parsing csv file : {:?}", e),
            }
        }

        room_data_map
    }
}
