use error_chain::error_chain;
use select::document::Document;
use select::predicate::Class;

error_chain! {
      foreign_links {
          ReqError(reqwest::Error);
          IoError(std::io::Error);
      }
}

// Source: https://rust-lang-nursery.github.io/rust-cookbook/web/scraping.html#extract-all-links-from-a-webpage-html
#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("https://www.cdc.gov/diseasesconditions/az/a.html")
        .await?
        .text()
        .await?;

    Document::from(res.as_str())
        .find(Class("unstyled-list"))
        .next() // Get the first match
        .expect("no matching <ol>")
        .children()
        .for_each(|i| print!("{};", i.text()));
    Ok(())
}
