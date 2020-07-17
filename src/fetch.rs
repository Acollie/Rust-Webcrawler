extern crate soup;

use soup::*;

pub(crate) fn fetch_page(url:&String) ->Soup{
    println!("page url:{}",&url);
    let page=reqwest::get(url).expect("Error parsing page").text().expect("");
    return Soup::new(&page);
}

#[cfg(test)]
mod test{
    use crate::fetch::fetch_page;
    use soup::{QueryBuilderExt, NodeExt};

    #[test]
    fn test_fetch_page(){
        let page=fetch_page(&"http://example.com/".to_string()).tag("title").find().unwrap();

        assert_eq!(page.text(),"Example Domain")
    }
}