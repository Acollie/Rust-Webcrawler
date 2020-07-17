
#[macro_use] extern crate serde_derive;

extern crate url;
extern crate reqwest;
extern crate soup;
extern crate regex;

mod web_page_format;
mod fetch;
mod file_management;

use std::collections::LinkedList;
use std::borrow::Borrow;

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
    search_space(&"https://www.bbc.co.uk".to_string(),1);
    // file_management::save_file_sweep();

}
