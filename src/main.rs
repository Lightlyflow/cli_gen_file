use std::fs::File;
use std::io;
use std::iter::successors;
use clap::Parser;
use String;

#[derive(Parser)]
#[clap(name = "File Generator")]
#[clap(author = "Lightlyflow")]
#[clap(about = "Generates a number of files with the same root name.")]
struct Cli {
    /// Root name of the files that will be created
    name: String,
    /// Number of files that will be created
    count: u64,
    /// Starting number
    #[clap(short, long, default_value_t = 0, value_parser)]
    start: u64,
    /// Add a file type
    #[clap(short='f', long, value_parser)]
    file_type: Option<String>,
    /// Add padding to file numbers
    #[clap(short, long, action)]
    padding: bool,
}

impl Cli {
    fn show_args(&self) {
        println!("Args:");
        println!("\tName: {}", self.name);
        println!("\tStart: {}", self.start);
        println!("\tCount: {}", self.count);
        println!("\tFile Type: {}", self.file_type.as_ref().unwrap_or(&"None".to_owned()));
        println!("\tPadding: {}", self.padding);
    }

    fn run(&self) -> Result<(), io::Error> {
        let base_file_name= self.name.clone();
        let mut file_name;
        let file_ending = match &self.file_type {
            None => { String::from("") }
            Some(s) => { format!(".{s}") }
        };
        let mut index: String;
        let max_pad = successors(Some(self.start + self.count - 1), |&n| (n >= 10).then(|| n / 10)).count();

        for i in self.start..self.start+self.count {
            // Formatting
            if self.padding {
                index = format!("{:0max_pad$}", i, max_pad=max_pad);
            } else {
                index = i.to_string();
            }
            file_name = format!("{base_file_name}{index}{file_ending}");

            println!("Creating {file_name}...");
            if std::path::Path::new(&file_name).exists() {
                eprintln!("\t{file_name} already exists.");
                continue;
            }

            File::create(file_name)?;
        }
        Ok(())
    }
}


fn main() {
    let cli = Cli::parse();

    // cli.show_args();
    match cli.run() {
        Ok(_) => { println!("Done.") },
        Err(e) => { eprintln!("{e}") },
    }
}
