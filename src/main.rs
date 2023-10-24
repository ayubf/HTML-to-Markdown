use std::env;
use std::process::Command;
use std::process::exit;
use regex::Regex;

extern crate fstrings;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_re = Regex::new(r".*.html").unwrap();

    if args.len() < 2 {
        println!("Please give an input file for translation\n");
        exit(0);
    }

    let Some(_) = file_re.captures(&args[1]) else {
        println!("Please give a valid input file for translation");
        exit(0);
    };

    let tidy_check = match Command::new("tidy")
        .arg("-v")
        .output() {
            Ok(output) => {
                if output.status.success() {
                    true

                } else {
                    false
                }
            },
            Err(_) => {
                false
            }
        };
    
    if !tidy_check {
        println!("Please install the latest version of Tidy.\nFor more info visit: https://www.html-tidy.org/#:~:text=Install");
        exit(0);
    }

    let file_output = match Command::new("tidy")
        .args(&["-q", &args[1]])
        .output() {
            Ok(o) => {
                o.stdout
            },
            Err(why) => {
                println!("{}", why);
                exit(0);
            },
    };

    let output_string: String = String::from_utf8(file_output).unwrap();
    
    let line_vec: Vec<&str> = output_string
        .split("\n")
        .map(|e| e.trim())
        .collect();

    for i in line_vec {
        println!("{}", i);
    }

}
