use rand::{self, Rng};
use scraper::{Html, Selector};
// use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://en.wikipedia.org/wiki/List_of_Intel_codenames").await?;
    let text = resp.text().await?;

    // let text = fs::read_to_string("./tmp/code_names.html").await?;
    // println!("{}", text);

    let document = Html::parse_document(&text);
    let selector = Selector::parse(r#".wikitable.sortable tbody tr td:first-child"#).unwrap();

    let mut code_names: Vec<String> = Vec::new();

    for td in document.select(&selector) {
        let codename = td.text().collect();
        code_names.push(codename);
    }

    let choice = &code_names[rand::thread_rng().gen_range(0..code_names.len())];
    println!("{}", choice.trim().to_lowercase());
    Ok(())
}
