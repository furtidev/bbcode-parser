use fancy_regex::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let re = Regex::new(r"^\[(.+).*\](.*?)\[\/\1\]$").unwrap();
    let test = re.is_match(&args[1]);

    match test {
        Ok(r) => {
            if r == false {
                println!("\x1b[91mPattern is not matching.\x1b[0m");
            } else {
                println!("\x1b[92mPattern is matching.\x1b[0m");
                let captures = re.captures(&args[1]).unwrap().unwrap();
                println!("\x1b[96mBBCODE:\x1b[0m \x1b[93m{:?}\x1b[0m", captures.get(0).unwrap().as_str());
                let captured_values = (captures.get(1).unwrap().as_str(), captures.get(2).unwrap().as_str());
                println!("\x1b[96mCaptured Values:\x1b[0m \x1b[93m{:?}\x1b[0m", captured_values);
                println!("\x1b[96mRendered HTML:\x1b[0m \x1b[93m<{}>{}</{}>\x1b[0m", captured_values.0, captured_values.1, captured_values.0);
            }
        },
        Err(e) => {
            println!("ERROR: {}", e);
        },
    }
}