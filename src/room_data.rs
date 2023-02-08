pub mod room_data {
    use csv;
    use std::collections::HashMap;
    use std::fs::File;
    use std::path::Path;

    fn parse_csv() -> csv::Reader<File> {
        let csv_path = Path::new("./data/super_metroid_rooms.csv");
        match File::open(csv_path) {
            Ok(csv_file) => return csv::Reader::from_reader(csv_file),
            Err(e) => panic!("Error parsing csv file : {:?}", e),
        }
    }

    pub fn room_data_gen() -> HashMap<String, String> {
        let mut csv_reader = parse_csv();

        let mut room_data_map: HashMap<String, String> = HashMap::new();
        for result in csv_reader.records() {
            match result {
                Ok(record) => {
                    room_data_map.insert(record[0].to_owned(), record[3].to_owned());
                }
                Err(e) => panic!("Error parsing csv file : {:?}", e),
            }
        }

        room_data_map
    }
}
