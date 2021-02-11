use std::env::args;

fn main() {
    for a in args(){
        if let Some(c) = a.chars().next() {
            match c {
                'e'|'E'=> println!("Merhaba {}", a),
                _=>{}
            }
        }
    }
}