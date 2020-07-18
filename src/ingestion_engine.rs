use soup::{Soup, QueryBuilderExt, NodeExt};
use std::collections::LinkedList;

// From https://en.wikipedia.org/wiki/Most_common_words_in_English


struct site{
    title:String,
    website_type: website_types,
    words:LinkedList<Word>
}

struct Word{
    word:String,
    hits:i32,
}
enum website_types{
    personal_site,
    shopping,
    travel,

}
enum common_locations{
    h1,
    h2,
    body,
    p
}
pub fn words_from_soup(soup:Soup){
    let test=soup.tag("h1").find_all();
    let h1=soup.tag("h1").find_all().count() as i32;
    let h2=soup.tag("h2").find_all().count() as i32;
    let p=soup.tag("p").find_all().count() as i32;
    let body=soup.tag("body").find_all().count() as i32;




}

fn words()->Vec<String>{
    let words:Vec<String> = vec!["the".to_string(),
        "be".to_string(),
        "to".to_string(),
        "of".to_string(),
        "in".to_string(),
        "a".to_string(),
        "in".to_string(),
        "that".to_string(),
        "I".to_string(),
        "it".to_string(),
        "for".to_string(),
        "not".to_string(),
        "on".to_string(),
        "with".to_string(),
        "he".to_string(),
        "as".to_string(),
        "as".to_string(),
    ];
    return words;
}