#[cfg(not(target_os = "emscripten"))]
use std::env;

#[cfg(not(target_os = "emscripten"))]
use std::fs;


#[cfg(not(target_os = "emscripten"))]
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 1 && args[1].as_str() == "run" {
        let filename = &args[2].split('.').collect::<Vec<_>>();
        if filename[filename.len() - 1] != "syn" {
            println!("File must have the extention .syn");
            return;
        }
        let content = fs::read_to_string(&args[2]).expect("Could not read file.");

        synthia::interpret(content.as_str());
    } else {
    
        println!(
            "Welcome to the gui of Synthia .",
        );
        synthia::repl::start();
    }

}

