extern crate serde_yaml;


use std::time::SystemTime;
use std::fs::File;
use std::collections::BTreeMap;
use crate::main;
use std::io::Write;
use std::ops::Add;
use std::borrow::Borrow;
use std::collections::LinkedList;

use crate::web_page_format::Page;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Sweep_info{
    date:i64,
    sites: LinkedList<Page>
}
pub fn save_file_sweep(sites:LinkedList<Page>){

    File::create("sites.yaml");
    let epoch_time:i64;
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(time)=>{
            epoch_time= time.as_secs() as i64;
        }
        _ => {panic!(1)}
    }
    let sweep=Sweep_info{
        date: epoch_time,
        sites: sites
    };
    let format_sweep = serde_yaml::to_string(&sweep).unwrap();

    let mut file=File::create("sites.yaml");
    match file {
        Ok(mut file)=>{
            file.write_all(format_sweep.as_bytes());
        }
        _ => {}
    }
}