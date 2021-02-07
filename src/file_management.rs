extern crate serde_yaml;
extern crate yaml_rust;


use std::time::SystemTime;
use std::fs::File;
use std::fs;
use std::io::{Write};

use yaml_rust::YamlLoader;
use crate::web_page_format::Page;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct SweepInfo {
    date:i64,
    sites: Vec<Page>
}

pub struct Settings{
    pub start_site:String,
    pub sweep_depth:i32,

}
pub fn load_config()->Settings{
    let file= fs::read_to_string("config.yaml").unwrap();
    let yaml= YamlLoader::load_from_str(&file).unwrap();

    return Settings{
        start_site: yaml[0]["start_site"].as_str().unwrap().to_string(),
        sweep_depth: yaml[0]["sweep_depth"].as_i64().unwrap() as i32
    }

}


pub fn save_file_sweep(sites:Vec<Page>){

    File::create("sites.yaml").expect("Failed to create file");
    let epoch_time:i64;
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(time)=>{
            epoch_time= time.as_secs() as i64;
        }
        _ => {panic!(1)}
    }
    let sweep= SweepInfo {
        date: epoch_time,
        sites
    };
    let format_sweep = serde_yaml::to_string(&sweep).unwrap();

    let file=File::create("sites.yaml");
    match file {
        Ok(mut file)=>{
            match file.write_all(format_sweep.as_bytes()){
                Ok(_)=>{
                    println!("Saved to file");
                }
                Err(e)=>{
                    println!("Error during save ERROR:{}",e);
                }
            }
        }
        _ => {}
    }
}