extern crate soup;

use soup::*;

pub(crate) fn fetch_page(url:&String) ->Soup{
    println!("page url:{}",&url);
    let page=reqwest::get(url).expect("Error parsing page").text().expect("");
    return Soup::new(&page);
}

