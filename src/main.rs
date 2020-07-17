
#[macro_use] extern crate serde_derive;

extern crate url;
extern crate reqwest;
extern crate soup;
extern crate regex;

mod web_page_format;
mod fetch;
mod file_mangament;

use soup::*;
use reqwest::{get};
use url::*;
use regex::*;
use std::collections::LinkedList;
use std::borrow::Borrow;
use std::{thread, time};
use web_page_format::Page;
use web_page_format::soup_to_links;
use std::convert::TryInto;
use std::fs::remove_file;

fn search_space(first_item:&String,depth:u8){
    let mut links_to_explore:Vec<String> = Vec::new();
    let mut visited_nodes:LinkedList<String> = LinkedList::new();
    let mut last_node:String = "START NODE".to_string();
    links_to_explore.push(first_item.parse().unwrap());
    let mut nodes = LinkedList::new();

    let mut counter= 0;

    loop {
        if counter==links_to_explore.len() {break;}
        let link = &links_to_explore[counter];
        let page = fetch::fetch_page(&link);
        for link in web_page_format::soup_to_links(&page, &link) {
            if !links_to_explore.contains(link.clone().borrow()){
                links_to_explore.push(link.clone());
            }
            if !visited_nodes.contains(link.clone().borrow()){
                visited_nodes.push_back(link.clone());
            }
            nodes.push_back(web_page_format::soup_page_formater(&page,last_node,link.clone().to_string()));

            last_node=link;
        }

        if counter == 0 {
            file_mangament::save_file_sweep(nodes);
            break

        }
    }
    println!("Visted all_nodes");
    println!("Nodes visited:{}",visited_nodes.len())

}

fn main() {
    search_space(&"https://www.bbc.co.uk".to_string(),3);
    // file_mangament::save_file_sweep();

}
