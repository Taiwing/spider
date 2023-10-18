use std::path::PathBuf;
use std::fs;
use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about = "Extract every image from a given URL")]
struct Cli {
	/// URL to download the images from
	url: String,
	/// Recursively download the images in a URL received as a parameter
	#[arg(short, long)]
	recursive: bool,
	/// Set the maximum recursion level
	#[arg(short, long, value_name = "N", default_value = "5")]
	level: Option<u32>,
	/// Path where the images will be saved
	#[arg(short, long, value_name = "PATH", default_value = "./data")]
	path: Option<PathBuf>,
}

fn main() -> std::io::Result<()> {
	let args = Cli::parse();
	println!("url: {}", args.url);
	println!("recursive: {}", args.recursive);
	println!("level: {}", args.level.unwrap());

	fs::create_dir_all(args.path.unwrap())?;
	Ok(())
}
