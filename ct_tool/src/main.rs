
use clap::Parser;
#[derive(Debug, Parser)]
#[command(name = "Cut Tool", version, about = "A tool for cutting things", long_about = None)]
struct Args {

}

fn main() {
    let args = Args::parse();
    
    println!("Hello, world!");
}
