#![allow(unused)]

use std::{env, path::Path, process::exit};
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage:jmp <directory>");
        exit(0);
    }

    let wanted_dir = args[1].as_str();

    for data in WalkDir::new("/").into_iter().filter_map(|e| e.ok()) {  /* iterate over and ignore errors */
       
        if data.path().is_dir() {                                       /* check directory */
            
            let s = String::from(data.path().display().to_string());    /* path to String */
            let last = s.split("/").last().unwrap();                    /* split over "/" */

            if last.eq(wanted_dir) {                                    /* check passed data to required data */
                println!("{}", data.path().display().to_string().trim());
                exit(0);
            }

        }
    }
    println!("OOpS !!! {}", wanted_dir);
}
