extern crate clap;
use clap::{Arg, App, SubCommand};
use uuid::Uuid;

fn main() {
    let matches = App::new("rs uuid generator")
                                            .version("1")
                                            .author("norayya")
                                            .about("a random uuid generator.")
                                            .subcommand(SubCommand::with_name("new")
                                                                    .about("create new uuid string")
                                                                    .version("1")
                                                                    .author("norayya")
                                                                    .arg(Arg::with_name("count")
                                                                            .short("c")
                                                                            .long("count")
                                                                            .takes_value(true)
                                                                            .required(false)
                                                                            .default_value("1")
                                                                            .help("how many uuids do you want?")
                                                                        )
                                                        )
                                            .subcommand(SubCommand::with_name("try")
                                                                    .about("parse a string and check whether it is a uuid")
                                                                    .version("1")
                                                                    .author("norayya")
                                                                    .arg(Arg::with_name("uuid")
                                                                            .short("u")
                                                                            .long("uuid")
                                                                            .takes_value(true)
                                                                            .required(true)
                                                                            .help("uuid string")
                                                                )
                                                        )
                                            .get_matches();

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

}
