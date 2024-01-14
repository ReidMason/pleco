mod args;

use std::collections::HashMap;

use args::{Command, PlecoArgs};
use clap::Parser;
extern crate glob;
use self::glob::glob;

fn main() {
    let args = PlecoArgs::parse();

    match args.command {
        Command::ListCommon(x) => list_common(&x.filepath),
        Command::Count(x) => count(&x.filepath, &x.search),
    }
}

fn count(filepath: &str, search: &str) {
    let paths = glob(&format!("{}/**/{}", filepath, search)).unwrap();

    println!(
        "Found {} occurances of '{}' in '{}'",
        paths.count(),
        search,
        filepath
    );
}

fn list_common(filepath: &str) {
    let paths = glob(&format!("{}/**/*", filepath)).unwrap();

    let mut file_types: HashMap<String, usize> = HashMap::new();

    for path in paths {
        let path = path.unwrap();
        let extension = match path.extension() {
            Some(ext) => ext.to_str().unwrap().to_string(),
            None => String::from("Unknown"),
        };

        let count = file_types.entry(extension).or_insert(0);
        *count += 1;
    }

    println!("Common file types found:");
    let mut file_types: Vec<_> = file_types.into_iter().collect();
    file_types.sort_by(|a, b| b.1.cmp(&a.1));

    for (file_type, count) in file_types.iter().take(5) {
        println!("  {}: {}", file_type, count);
    }
}
