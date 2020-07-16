#![feature(option_result_contains)]

extern crate url;
extern crate reqwest;
extern crate soup;
extern crate regex;

mod web_page_format;

use soup::*;
use reqwest::{get};
use url::*;
use regex::*;
use std::collections::LinkedList;
use std::borrow::Borrow;
use std::ptr::null;
use std::{thread, time};


fn search_space(x:&String){
    let mut links_to_explore:Vec<String> = Vec::new();
    let mut visited_nodes:LinkedList<String> = LinkedList::new();
    let mut last_node:String = "START NODE".to_string();
    links_to_explore.push(x.parse().unwrap());
    let mut nodes = LinkedList::new();


    let mut counter= 0;
    loop {
        if counter==links_to_explore.len() {break;}
        let link = &links_to_explore[counter];
        let page = fetch_page(&link);
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
        counter+=1;
        if counter%5==0{
            println!("Links");
            for link in &links_to_explore{
                println!("{}",link);
            }
            println!("page info");
            let new_node=nodes;
            for page in new_node{
                web_page_format::print_page_info(page);
            }
            break

        }
    }
    println!("Visted all_nodes");
    println!("Nodes visited:{}",visited_nodes.len())


}

fn fetch_page(url:&String)->Soup{
    println!("page url:{}",&url);
    let page=reqwest::get(url).expect("Error parsing page").text().expect("");
    return Soup::new(&page);
}

fn main() {
    search_space(&"https://www.bbc.co.uk".to_string());


}
