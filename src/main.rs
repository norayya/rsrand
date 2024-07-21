extern crate clap;

use std::{fs::File, io::Read};

use clap::{App, YamlLoader};
use uuid::Uuid;
use yaml_rust::Yaml;

fn load(path: &str) -> Yaml {
    let mut f = File::open(path).expect("Failed to open config YAML file");
    let mut yaml_str = String::new();
    
    f.read_to_string(&mut yaml_str).expect("Failed to read config YAML file");


    let yaml = YamlLoader::load_from_str(&yaml_str)
        .expect("Failed to load config YAML file");

    yaml[0].clone()
}
fn main() {
    let yaml = load("cli.yaml");
    let matches = App::from_yaml(&yaml).get_matches();

    match matches.subcommand() {
        ("uuid", Some(matches)) => {
            match matches.subcommand() {
                ("new", Some(matches)) => {
                    if matches.is_present("count"){
                        let count:Result<i32, _>= matches.value_of("count").unwrap().parse();
                        match count {
                            Ok(num) => {
                                for _ in 0..num {
                                    let uuid = Uuid::new_v4();
                                    println!("{uuid}");
                                }
                            },
                            Err(err) => println!("some error has occurred : {}", err),
                        }
                    }else{
                        println!("cannot take any args.. failed.");
                    }
                },
                ("try", Some(matches))=>{
                    if matches.is_present("uuid"){
                        let parse_result = Uuid::try_parse(matches.value_of("uuid").unwrap());
                        match parse_result {
                            Ok(_) => println!("success"),
                            Err(err) => println!("some error has occurred : {}", err),
                        }
        
                    }else{
                        println!("cannot take any args.. failed.")
                    }
                },
                _ => println!("command not found.. failed.")
            }
        
        },
        _ => println!("command not found.. failed.")
    }


}
