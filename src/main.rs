

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

use std::collections::HashSet;


fn search_space(first_item:String,depth:i32 ){
    let mut links_todo=Vec::new();
    let mut links_done:HashSet<String>=HashSet::new();
    let mut nodes=Vec::new();
    let mut current_depth = 0;
    links_todo.push((first_item, String::from("START_NODE")));


    while let Some((page_url, last_page)) = links_todo.pop() {
        if current_depth >= depth {
            break
        }
        current_depth += 1;
        let page= fetch::fetch_page(&page_url);
        let links= web_page_format::soup_to_links(&page, &*page_url);

        nodes.push(web_page_format::soup_page_formatter(&page, last_page.to_string(), &page_url));
        for link in links {
            if !links_done.contains(&link){
                links_done.insert(link.clone());
                links_todo.push((link, page_url.clone()));
            }
        }
    }
    file_management::save_file_sweep(nodes);
}

fn main() {

    let settings = file_management::load_config();
    search_space(settings.start_site, settings.sweep_depth);

}
