pub mod settings {
    use std::io;
    use std::fs;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    pub struct Config {
        pub pushover_key: String,
        pub rss_url: String
    }

    impl Config {
        pub fn get_config(filename: &str) -> Result<Config, io::Error> {
            let yaml_string = fs::read_to_string(filename)?;
            let config: Config = serde_yaml::from_str(yaml_string.as_str()).unwrap();
            Ok(config)
        }

        pub fn write_stock_yaml(path: &str) {
            let stock = Config{pushover_key: String::from("Please fill in your pushover key"), rss_url: String::from("https://www.bchydro.com/rss/outages/all.xml")};
            let yaml = serde_yaml::to_string(&stock).unwrap();
            fs::write(path, yaml).expect("Failed to write stock file");
        }
    }
}