use serde_json;
use std::error::Error;

pub struct Config {
    pub isbn: String,
}

impl Config {
    pub fn new(input: &Vec<String>) -> Result<Config, &str> {
        if input.len() != 2 {
            return Err("You can use a single argument")
        }
        let arg1 = input[1].clone();
        Ok(Config{isbn: arg1})
    }
}

struct BookInfo {
    title: String,
    subtitle: String,
    authors: String,
    publisher: String,
    published_date: String,
    page_cnt: String,
    category: String,
    isbn13: String,
    isbn10: String,
    description: String,
}

impl BookInfo {
    // TODO:
    // - err type
    fn new(v: &serde_json::Value) -> Result<BookInfo, &str> {
        let vi = &v["volumeInfo"];
        Ok(BookInfo {
            title: vi["title"].to_string(),
            subtitle: vi["subtitle"].to_string(),
            authors: vi["authors"].to_string(),
            publisher: vi["publisher"].to_string(),
            published_date: vi["publishedDate"].to_string(),
            category: vi["category"].to_string(),
            page_cnt: vi["page_cnt"].to_string(),
            isbn13: BookInfo::get_isbn_13(&vi).unwrap(),
            isbn10: vi["isbn10"].to_string(),
            description: vi["description"].to_string(),
        })
    }
    fn print_exist(target: &str, val: &String) {
        if val != "null" {
            println!("{target}: {val}");
        }
    }
    fn print_bookinfo(bi: &BookInfo) { 
        BookInfo::print_exist("Title", &bi.title);
        BookInfo::print_exist("Sub Title", &bi.subtitle);
        BookInfo::print_exist("Authors", &bi.authors);
        BookInfo::print_exist("Category", &bi.category);
        BookInfo::print_exist("Publisher", &bi.publisher);
        BookInfo::print_exist("Published Date", &bi.published_date);
        BookInfo::print_exist("Page Count", &bi.page_cnt);
        BookInfo::print_exist("ISBN 13", &bi.isbn13);
        BookInfo::print_exist("ISBN 10", &bi.isbn10);
        BookInfo::print_exist("Description", &bi.description);
    }
    // TODO type of serde_json::Value
    // https://docs.rs/serde_json/latest/serde_json/enum.Value.html
    fn get_isbn_13(v: &serde_json::Value) -> Option<String> {
        let identifier = &v["industryIdentifiers"];
        for isbns in identifier.as_array().unwrap() {
            if isbns["type"].as_str() == Some("ISBN_13") {
                return Some(isbns["identifier"].as_str().unwrap().to_string());
            }
        }
        return None;
    }
}

/*
 * TODO: Box<dyn Error> means the function will return a type that implements the Error trait
 */
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut request_url = String::from("https://www.googleapis.com/books/v1/volumes?q=");
    request_url.push_str(&config.isbn);

    let body: String = reqwest::blocking::get(&request_url)
        .unwrap()
        .text()
        .unwrap();

    let v: serde_json::Value = serde_json::from_str(&body).unwrap();
    
    let bookinfo = BookInfo::new(&v["items"][0]).unwrap();
    BookInfo::print_bookinfo(&bookinfo);
    //println!("{}", body);
    let bookinfo = BookInfo::new(&v["items"][1]).unwrap();
    BookInfo::print_bookinfo(&bookinfo);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
         assert_eq!(1 + 1, 2);       
    }
    
    #[test]
    fn get_isbn_13_test() {
        let json_data = r#"
            {
                "industryIdentifiers": [
                  {
                    "type": "ISBN_13",
                    "identifier": "9788937431579"
                  },
                  {
                    "type": "ISBN_10",
                    "identifier": "8937431572"
                  }
                ]
            }"#;
        let data: serde_json::Value = serde_json::from_str(&json_data).unwrap();
        assert_eq!(
            BookInfo::get_isbn_13(&data),
            Some("9788937431579".to_string())
        );

        let json_data = r#"
            {
                "industryIdentifiers": [
                  {
                    "type": "ISBN_13",
                    "identifier": "9788937431579"
                  }
                ]
            }"#;
        let data: serde_json::Value = serde_json::from_str(&json_data).unwrap();
        assert_eq!(
            BookInfo::get_isbn_13(&data),
            Some("9788937431579".to_string())
        );
    }
}
