use rss::Channel;
use scraper::{Html, Selector, ElementRef};

struct Outage {
    guid: String,
    title: String,
    link: String,
    published: String,
    description: Option<Description>
}

struct Description {
    municipality: String,
    occurred_at: String,
    est_restore: String,
    num_customers: String,
    cause: String,
    area: String,
    updated_at: String
}

fn main() {
    let url = String::from("https://www.bchydro.com/rss/outages/all.xml");
    let feed = get_feed(&url);
    let mut outages: Vec<Outage> = Vec::new();

    // println!("{}", feed.to_string());
    let item_iter = feed.items().iter();

    for item in item_iter {
        let mut outage = Outage { guid: String::new(), title: String::new(), link: String::new(), published: String::new(), description: None };
        if let Some(guid) = item.guid() { outage.guid = String::from(guid.value()) }

        if let Some(title) = item.title() { outage.title = title.to_string() }

        if let Some(link) = item.link() { outage.link = link.to_string() }

        if let Some(published) = item.pub_date() { outage.published = published.to_string()}

        if let Some(desc) = item.description() {
            let description = parse_description(&String::from(desc));
                outage.description = Some(description);
        }

        println!("GUID: {}, Title: {}, Link: {}, Published: {}, Municipality: {}", outage.guid, outage.title, outage.link, outage.published, outage.description.as_ref().unwrap().municipality);
        outages.push(outage);
    }
}

fn get_feed(url: &String) -> Channel {
    let channel = Channel::from_url(url).expect("No network connection");
    channel
}

fn parse_description(desc: &String) -> Description {
    let table = Html::parse_fragment(desc);
    let row_selector = Selector::parse("tr").unwrap();

    let mut outage = Description{
        municipality: String::new(), 
        occurred_at: String::new(), 
        est_restore: String::new(),
        num_customers: String::new(), 
        cause: String::new(), 
        area: String::new(),
        updated_at: String::new()
    };

    for row in table.select(&row_selector) {
        let name = get_name(&row);
        let value = get_value(&row);

        match name.as_str() {
            "Municipality:" => outage.municipality = value,
            "Time Off:" => outage.occurred_at = value,
            "Est. Time On:" => outage.est_restore = value,
            "# Customers Affected:" => outage.num_customers = value,
            "Outage Type/Cause:" => outage.cause = value,
            "Approx. Area Affected:" => outage.area = value,
            "Last Updated:" => outage.updated_at = value,
            _ => ()
        }
    }
    outage
}

fn get_row_data_by_index(row: &ElementRef, index: usize) -> String {
    let data_selector = Selector::parse("td").unwrap();

    let nth = row.select(&data_selector).nth(index);
    let mut text = String::new();
    for data in nth {        
        let text_iter = data.text();
        for text_node in text_iter {
            text.push_str(text_node);
        }
    }
    text
}

fn get_name(row: &ElementRef) -> String {
    let name = get_row_data_by_index(row, 0);
    name
}

fn get_value(row: &ElementRef) -> String {
    let value = get_row_data_by_index(row, 1);
    value
}