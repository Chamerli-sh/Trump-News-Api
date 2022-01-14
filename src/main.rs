use std::error::Error;

struct Articles {
    articles: Vec<Article>
}

struct Article {
    title: String,
    url: String
}

fn get_article(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;

    dbg!(response);

    todo!()
}


fn main() {
    let url: &str = "https://newsapi.org/v2/top-headlines?q=trump&apiKey=API_KEY";
    let article = get_article(url);
}
