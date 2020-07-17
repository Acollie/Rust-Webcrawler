extern crate soup;
extern crate url;

use soup::*;
use std::vec::*;
use url::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Page {
    last_linker:String,
    title:String,
    about:String,
    url:String,
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
pub fn soup_page_formater(page:&Soup, last_page:String,page_url:String) -> Page {
    let mut current_page = Page {
        last_linker: "".to_string(),
        title: "".to_string(),
        about: "".to_string(),
        url:page_url,
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

