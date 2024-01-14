mod args;

use std::collections::HashMap;

use args::{Command, PlecoArgs};
use clap::Parser;

extern crate glob;
use glob::glob;

fn main() {
    let args = PlecoArgs::parse();

    match args.command {
        Command::ListCommon(x) => list_common(&x.filepath),
        Command::Count(x) => {
            count(&x.filepath, &x.search);
        }
    };
}

fn count(filepath: &str, search: &str) -> usize {
    let paths = glob(&format!("{}/**/{}", filepath, search)).unwrap();

    let path_count = paths.count();
    println!(
        "Found {} occurances of '{}' in '{}'",
        path_count, search, filepath
    );

    return path_count;
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

#[cfg(test)]
mod tests {
    use std::fs::File;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_count() {
        let dir = tempdir().unwrap();

        let test_files = vec![
            "test_dir/test_file1.txt",
            "test_dir/test_file2.rs",
            "test_dir/nested/test_file2.rs",
        ];

        let test_dir = dir.path().to_str().unwrap();
        for path in test_files.iter() {
            let file_path = dir.path().join(path);
            let prefix = file_path.parent().unwrap();
            std::fs::create_dir_all(prefix).unwrap();
            File::create(file_path).unwrap();
        }

        println!("test_dir: {}", test_dir);
        let result = count(test_dir, "test_file2.rs");
        assert_eq!(result, 2);
    }
}
