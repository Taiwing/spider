use std::path::PathBuf;
use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
	url: String,
	#[arg(short, long)]
	recursive: bool,
	#[arg(short, long, value_name = "N")]
	level: Option<u32>,
	#[arg(short, long, value_name = "PATH")]
	path: Option<PathBuf>,
}

fn main() {
	let args = Cli::parse();
	println!("url: {}", args.url);
	println!("recursive: {}", args.recursive);
	println!("level: {}", args.level.unwrap_or(0));
	println!("path: {}", args.path.unwrap_or(PathBuf::from("default")).display());
}
