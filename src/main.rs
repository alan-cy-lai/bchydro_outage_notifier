mod bchydro_rss;
mod settings;
use bchydro_rss::bchydro::Feed;
use settings::settings::Config;

const FILENAME: &str = "settings.yaml";

fn main() {
    let config = Config::get_config(FILENAME).expect("Unable to read config file");
    let feed = Feed::get_feed(config.rss_url.as_str());

    let all_outages_vec = Feed::get_all_outages(&feed);    
    let vancouver_outages = Feed::filter_municipality(&all_outages_vec, &String::from("Vancouver"));
}