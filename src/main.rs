use std::env;
use std::process::Command;
use std::process::exit;
use html2text::from_read_rich;
use regex::Regex;
use std::fs::File;
use std::fs;

fn file_exists(f: &String) -> bool {
    match Command::new("ls")
    .arg(f)
    .output() {
        Ok(o) => {
            if o.status.success() {
                return true
            } else {
                return false
            }
        }
        Err(_) => {
            return false
        }
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file_re = Regex::new(r".*.html").unwrap();
    let output_file_re = Regex::new(r".*.md").unwrap();

    if args.len() < 2 || args[1] == "-h" || args[1] == "--help" {
        println!("
            html_to_markdown [file...] [option...] [file...]
            
            File
            ----
            Must be valid .html file

            Flags
            -----
            -p or --print:
                Will output to console

            -o or --output:
                Will output to .md file of the same name as the input file
            
            -o [file...] or --output [file...]:
                Will output to given file

            Visit my github: https://github.com/ayubf
        
        ");
        exit(0);
    }

    let input_file_path = &args[1];
    
    if !input_file_re.is_match(&input_file_path) {
        println!("Please give a valid input file for translation\nUse html_to_markdown -h for help");
        exit(0);
    }

    let mut output_file_name: String = args[1][..5].to_string() + ".md";

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
    } else if !file_exists(input_file_path) {
        println!("Input file does not exist");
        exit(0);
    }

    let file_output = match Command::new("tidy")
        .args(&["-q", &input_file_path])
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

    let width = lines_vec
        .clone()
        .into_iter()
        .map(|i| {return i.len()})
        .max()
        .unwrap();

    let body = lines_vec
        .into_iter()
        .collect::<String>();

    let byte_str_body: &[u8] = &body.as_bytes();
    let body_as_text = from_read_rich(byte_str_body, width)
        .into_iter()
        .map(|i| {
            return i.into_string()
        })
        .collect::<Vec<String>>()
        .join("\n");


    if 2 <= args.len() && args.len() < 3 {
        println!("{}", body_as_text);
        exit(0);
    } else if args[2] == "-p" || args[2] == "--print" {
        println!("{}", body_as_text);
        exit(0);
    } else if args[2] == "-o" || args[2] == "--output" {
        if args.len() > 3 && output_file_re.is_match(&args[3]) {
            output_file_name = args[3].clone();
        } 
    }

    if output_file_name != "".to_string() && file_exists(&output_file_name) {
        match File::create(output_file_name.clone()) {
            Ok(_) => {}
            Err(_) => {}
        }
    }


    let _ = fs::write(output_file_name, body_as_text.as_bytes());


}
