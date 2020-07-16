
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
use web_page_format::page;
use web_page_format::soup_to_links;
use std::convert::TryInto;
use std::fs::remove_file;

fn search_space(first_item:&String){
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
            nodes.push_back(web_page_format::soup_page_formater(&page,last_node));

            last_node=link;
        }
        println!("Nodes explored:{}",visited_nodes.len());
        println!("Nodes  to explore:{}",links_to_explore.len());
        web_page_format::soup_page_formater(&page, last_node.clone());
        counter += 1;
        if counter%3 == 0 {
            web_page_format::display_links(visited_nodes.clone());
            break

        }
    }
    println!("Visted all_nodes");
    println!("Nodes visited:{}",visited_nodes.len())

}

fn main() {
    // search_space(&"https://www.bbc.co.uk".to_string());
    file_mangament::open_file();

}
