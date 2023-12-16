use chrono::Local;
use reqwest::{Client, StatusCode};
use scraper::{Html, Selector};
use serde::Serialize;
use std::fs::File;
use std::io::Write;

// Article struct
#[derive(Debug, Serialize, Clone)]
struct ArticleContent {
    article_title: String,
    url_link: String,
}

// Function to set app user agent.
fn get_client() -> Client {
    let agent = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    Client::builder().user_agent(agent).build().unwrap()
}
// Function to scrape website.
#[tokio::main]
async fn main() {
    // Define client for web request.
    let client = get_client();
    // Set domain name.
    let domain_name = "newsnationnow.com";
    // Define url for web request.
    let url = format!("https://{}", domain_name);
    // Request result from server.
    let result = client.get(&url).send().await.unwrap();
    // Validate response.
    let raw_html = match result.status() {
        StatusCode::OK => result.text().await.unwrap(),
        _ => panic!("Something went wrong fetching site"),
    };

    // Uncomment to save raw website html.
    //save_raw_html(&raw_html, domain_name);

    // Define Vec to store contents.
    let mut article_list: Vec<ArticleContent> = Vec::new();
    // Parse response string into html.
    let document = Html::parse_document(&raw_html);
    // Define Selector for desired content.
    let article_selector = Selector::parse("h3 > a").unwrap();

    // Iterate through site elements and isolate title and web address.
    for element in document.select(&article_selector) {
        let title = element.inner_html().trim().to_string();
        let href = element
            .value()
            .attr("href")
            .unwrap_or("no url found")
            .to_string();

        // Uncomment to print results to screen.
        // println!("Title: {:?}\nLINK: {}", title, href);

        // Push contents from scrape to Vec using defined Struct.
        article_list.push(ArticleContent {
            article_title: title,
            url_link: href,
        });
    }
    // Print number of articles scraped.
    println!("Number of articles scraped: {}", article_list.len());

    // Call function to write article Structs to json file.
    save_article_list(&article_list, domain_name);
}

// Function to write articles to json file.
fn save_article_list(article_list: &[ArticleContent], domain_name: &str) {
    // Define time of creation.
    let dtl = Local::now();
    // Define filename.
    let filename = format!("{}_{}.json", domain_name, dtl.format("%Y-%m-%d_%H.%M"));
    let mut writer = File::create(&filename).unwrap();
    write!(
        &mut writer,
        "{}",
        &serde_json::to_string(article_list).unwrap()
    )
    .unwrap();
}

// Function to write raw website results to an HTML file.
// fn save_raw_html(raw_html: &str, domain_name: &str) {
//     let dt = chrono::Local::now();
//     let filename = format!("{}_{}.html", domain_name, dt.format("%Y-%m-%d_%H.%M"));
//     let mut writer = File::create(&filename).unwrap();
//     write!(&mut writer, "{}", raw_html).unwrap();
// }
