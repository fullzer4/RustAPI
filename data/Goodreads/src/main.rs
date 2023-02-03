extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::error::Error;
use std::fs::File;
use std::io::Write;

#[derive(Debug, serde::Deserialize)]
struct Book {
    title: String,
    author: String,
    average_rating: String,
    ratings_count: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let key = "SUA_CHAVE_GOODREADS_API";

    let url = format!("https://www.goodreads.com/search/index.xml?key={}&q=Ender%27s+Game", key);
    let resp = reqwest::get(&url)?.text()?;

    let mut books = Vec::new();

    for line in resp.lines() {
        if line.contains("<title>") {
            let title = line[7..line.len()-8].to_string();
            books.push(Book {
                title,
                author: "".to_string(),
                average_rating: "".to_string(),
                ratings_count: "".to_string(),
            });
        } else if line.contains("<name>") {
            let author = line[6..line.len()-7].to_string();
            let last_book = books.last_mut().unwrap();
            last_book.author = author;
        } else if line.contains("<average_rating>") {
            let average_rating = line[16..line.len()-17].to_string();
            let last_book = books.last_mut().unwrap();
            last_book.average_rating = average_rating;
        } else if line.contains("<ratings_count>") {
            let ratings_count = line[15..line.len()-16].to_string();
            let last_book = books.last_mut().unwrap();
            last_book.ratings_count = ratings_count;
        }
    }

    let mut wtr = csv::Writer::from_path("books.csv")?;

    for book in books {
        wtr.write_record(book)?;
    }
    wtr.flush()?;

    Ok(())
}