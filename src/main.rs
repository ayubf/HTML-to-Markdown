use std::env;
use std::process::Command;
use std::process::exit;
use html2text::from_read;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_re = Regex::new(r".*.html").unwrap();

    if args.len() < 2 || !file_re.is_match(&args[1]) {
        println!("Please give a valid input file for translation");
        exit(0);
    } 

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

    let file_check = match Command::new("ls")
        .arg(&args[1])
        .output() {
            Ok(o) => {
                if o.status.success() {
                    true
                } else {
                    false
                }
            }
            Err(_) => {
                false
            }
    };
    
    if !tidy_check {
        println!("Please install the latest version of Tidy.\nFor more info visit: https://www.html-tidy.org/#:~:text=Install");
        exit(0);
    } else if !file_check {
        println!("File does not exist");
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
    
    let lines_vec: Vec<&str> = output_string
        .split("\n")
        .map(|e| e.trim())
        .collect();

    let body = lines_vec
        .into_iter()
        .collect::<String>();

    let byte_str_body: &[u8] = &body.as_bytes();
    let body_as_text = from_read(byte_str_body, 20);
    

}
