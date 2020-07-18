extern crate soup;
extern crate url;

use soup::*;
use std::vec::*;
use url::*;
use crate::ingestion_engine::Word;
use crate::ingestion_engine;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Page {
    last_linker:String,
    title:String,
    about:String,
    url:String,
    words:Vec<Word>
}

pub fn soup_to_links(soup:&Soup,base_url:&String) -> Vec<String> {
    let mut links=Vec::new();
    let all_links= soup.tag("a").find_all();

    for link in all_links{
        if !link.display().contains(":href")&& !link.display().contains("mailto") {
            if !link.get("href").is_none() {
                let tmp_link = link.get("href").expect("Error Parsing");
                if !tmp_link.contains("#") {
                    if tmp_link.contains("http://") || tmp_link.contains("https://") {
                        links.push(tmp_link);
                    } else {
                        let url = Url::parse(base_url).expect("");
                        links.push(url.join(&tmp_link).unwrap().to_string());
                    }
                }
            }
        }


    }
    return links;
}
//soup_page_formatter
pub fn soup_page_formatter(page:&Soup, last_page:String, page_url:String) -> Page {

    let mut current_page = Page {
        last_linker: "".to_string(),
        title: "".to_string(),
        about: "".to_string(),
        url:page_url,
        words: ingestion_engine::words_from_soup(&page).words
    };

    let title=page.tag("title").find().unwrap().text();
    current_page.title=title;
    current_page.last_linker=last_page;

    let h1 = page.tag("h1").find_all();
    let h2 = page.tag("h2").find_all();
    let mut body:String;
    body="".to_string();
    for x in h1{
        body.push_str(&x.text().to_string());
    }
    for x in h2{
        body.push_str(&x.text().to_string());
    }
    current_page.about=body;
    return current_page;

}

//Test
#[cfg(test)]
mod test{
    use crate::web_page_format::{soup_page_formatter, soup_to_links};
    use soup::Soup;

    #[test]
    fn test_soup_page_formatter(){
        let html=r#"
        <html><head><title>The Dormouse's story</title></head>
        <body>
        <p class="title"><b>The Dormouse's story</b></p>
        <h1>The Dormouse's story</h1>
        <p class="story">Once upon a time there were three little sisters; and their names were
        <a href="http://example.com/elsie" class="sister" id="link1">Elsie</a>,
        <a href="http://example.com/lacie" class="sister" id="link2">Lacie</a> and
        <a href="http://example.com/tillie" class="sister" id="link3">Tillie</a>;
        and they lived at the bottom of a well.</p>

        <p class="story">...</p>
        "#;
        let soup= Soup::new(html);
        let page= soup_page_formatter(&soup,"test.com".to_string(),"Test title".to_string());
        assert_eq!(page.last_linker,"test.com");
        assert_eq!(page.title,"The Dormouse's story");
        assert_eq!(page.about.contains("The Dormouse's story"),true);
        assert_eq!(page.about.contains("This does not appear"),false);

    }
    #[test]
    fn test_soup_to_links(){
        let html=r#"
        <html><head><title>The Dormouse's story</title></head>
        <body>
        <p class="title"><b>The Dormouse's story</b></p>
        <h1>The Dormouse's story</h1>
        <p class="story">Once upon a time there were three little sisters; and their names were
        <a href="/news" class="sister" id="link1">Elsie</a>,

        <a href="http://example.com/elsie" class="sister" id="link1">Elsie</a>,
        <a href="http://example.com/lacie" class="sister" id="link2">Lacie</a> and
        <a href="http://example.com/tillie" class="sister" id="link3">Tillie</a>;
        and they lived at the bottom of a well.</p>

        <p class="story">...</p>
        "#;
        let soup=Soup::new(html);
        let links=soup_to_links(&soup, &"http://example.com".to_string());
        assert_eq!(links.contains(&"http://example.com/elsie".to_string()),true);
        assert_eq!(links.contains(&"http://example.com/news".to_string()),true);
        assert_eq!(links.contains(&"/news".to_string()),false);

        for link in links{
            assert_eq!(link.contains("http"),true)
        }

    }
}
