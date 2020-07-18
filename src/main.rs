

#[macro_use]
extern crate serde_derive;
extern crate url;
extern crate reqwest;
extern crate soup;
extern crate regex;

mod web_page_format;
mod fetch;
mod file_management;
mod ingestion_engine;

use std::collections::LinkedList;
use std::borrow::Borrow;
use soup::Soup;


fn search_space(first_item:&String,depth:i32 ){
    let mut links_to_explore:Vec<String> = Vec::new();
    let mut visited_nodes:LinkedList<String> = LinkedList::new();
    let mut last_node:String = "START NODE".to_string();
    links_to_explore.push(first_item.parse().unwrap());
    let mut nodes = LinkedList::new();

    let mut counter:i32= 0;

    loop {
        if counter==links_to_explore.len() as i32 {break;}
        let link = &links_to_explore[counter as usize];
        let page = fetch::fetch_page(&link);
        for link in web_page_format::soup_to_links(&page, &link) {
            if !links_to_explore.contains(link.clone().borrow()){
                links_to_explore.push(link.clone());
            }
            if !visited_nodes.contains(link.clone().borrow()){
                visited_nodes.push_back(link.clone());
            }
            nodes.push_back(web_page_format::soup_page_formatter(&page, last_node, link.clone().to_string()));

            last_node=link;
        }

        if counter == depth {
            file_management::save_file_sweep(nodes);
            break

        }
        counter+=1;
    }
    println!("Visted all_nodes");
    println!("Nodes visited:{}",visited_nodes.len())

}

fn main() {
    // let settings=file_management::load_config();
    // search_space(&settings.start_site,settings.sweep_depth);
    let html=r#"
        <html><head><title>The Dormouse's story</title></head>
        <h1>How to fix your car</h1>
        <p>Many things might be useful when trying to fix your car but the most likely is that the car is broken because it has no fuel.</p>
        </html>
        "#;
    let soup=Soup::new(html);
    let test:ingestion_engine::Site=ingestion_engine::words_from_soup(soup);
    for x in test.words{
        println!("word :{}, hits :{}",x.word, x.hits)
    }
}
