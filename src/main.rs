// jkcoxson
// Simple tool for analyzing hex dumps

use colored::*;
use std::io::Write;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    let mut compact = false;
    for i in 0..args.len() {
        if i < args.len() {
            if args[i] == "--compact" || args[i] == "-c" {
                compact = true;
                args.remove(i);
            }
        }
    }
    if args.len() < 2 {
        panic!("\n\nNot enough arguments, supply a file!\n\n");
    }
    match std::fs::read(&args[1]) {
        Ok(bytes) => {
            if &args.len() < &3 {
                for i in bytes {
                    if i == 10 {
                        // newline character I guess
                        print!("0A ");
                    } else {
                        print!("{:X} ", i);
                    }
                }
                print!("\n");
                std::io::stdout().flush().unwrap();
            } else {
                match std::fs::read(&args[2]) {
                    Ok(compare_bytes) => {
                        let chonk_size = match bytes.len() > compare_bytes.len() {
                            true => bytes.len(),
                            false => compare_bytes.len(),
                        };

                        let mut same_up: Vec<u8> = vec![];

                        let mut up_stop: Option<usize> = None;

                        // Check going up the bytes
                        for i in 0..chonk_size {
                            match up_stop {
                                Some(_) => {}
                                None => {
                                    if bytes[i] == compare_bytes[i] {
                                        same_up.push(bytes[i]);
                                    } else {
                                        up_stop = Some(i);
                                    }
                                }
                            }
                        }
                        // If it never stopped reading up the file break immediately
                        if up_stop == None {
                            if compact {
                                println!("Same file");
                            } else {
                                println!("{}", get_hex_string(bytes).blue());
                                println!("{}", get_hex_string(compare_bytes).blue());
                            }
                            return;
                        }

                        let mut same_down: Vec<u8> = vec![];

                        let mut down_stop: Option<usize> = None;

                        // Check going down the bytes
                        let mut rev_bytes = bytes.clone();
                        rev_bytes.reverse();
                        let mut rev_compare = compare_bytes.clone();
                        rev_compare.reverse();
                        for i in 0..chonk_size {
                            match down_stop {
                                Some(_) => {}
                                None => {
                                    if rev_bytes[i] == rev_compare[i] {
                                        same_down.push(rev_bytes[i]);
                                    } else {
                                        down_stop = Some(i);
                                    }
                                }
                            }
                        }
                        same_down.reverse();

                        // Print out our results
                        if compact {
                            println!("Size difference: {}", same_up.len() - same_down.len());
                            print!("{} ", same_up.len().to_string().blue());
                            print!("{} ", chonk_size - same_up.len() - same_down.len());
                            print!("{} ", same_down.len().to_string().green());
                            print!("\n");
                            std::io::stdout().flush().unwrap();
                            return;
                        }
                        print!("{} ", get_hex_string(same_up.clone()).blue());
                        for i in up_stop.unwrap()..(chonk_size - down_stop.unwrap()) {
                            if bytes.len() > i {
                                print!("{:X} ", bytes[i])
                            }
                        }
                        print!("{}", get_hex_string(same_down.clone()).green());
                        print!("\n");
                        std::io::stdout().flush().unwrap();

                        // Print the comparing line
                        print!("{} ", get_hex_string(same_up).blue());
                        for i in up_stop.unwrap()..(chonk_size - down_stop.unwrap()) {
                            if compare_bytes.len() > i {
                                print!("{:X} ", compare_bytes[i])
                            }
                        }
                        print!("{}", get_hex_string(same_down).green());
                        print!("\n");
                        std::io::stdout().flush().unwrap();
                    }
                    Err(_) => {
                        panic!("\n\nBad file!\n\n");
                    }
                }
            }
        }
        Err(_) => {
            panic!("\n\nBad file!\n\n")
        }
    }
}

fn get_hex_string(vec: Vec<u8>) -> String {
    let mut first = true;
    let mut to_return = "".to_string();
    for i in vec {
        if first {
            to_return = format!("{:X}", i);
            first = false;
        } else {
            to_return = format!("{} {:X}", to_return, i);
        }
    }
    return to_return;
}
