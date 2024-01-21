mod args;

use std::{collections::HashMap, io::Write};

use args::{Command, PlecoArgs};
use clap::Parser;

extern crate glob;
use glob::glob;
use tabwriter::TabWriter;

fn main() {
    let args = PlecoArgs::parse();

    match args.command {
        Command::ListCommon(x) => {
            let file_types = get_common_filetypes(&x.filepath);

            println!("Common file types found:");
            let mut file_types_vec: Vec<_> = file_types.into_iter().collect();
            file_types_vec.sort_by(|a, b| b.1.cmp(&a.1));

            let mut output = String::from("");
            for (file_type, count) in file_types_vec.iter().take(5) {
                output += &String::from(format!("{}\t{}\n", file_type, count));
            }
            print_columns(&output);
        }
        Command::Count(x) => {
            count(&x.filepath, &x.search);
        }
    };
}

fn print_columns(output: &str) {
    let mut tw = TabWriter::new(vec![]);
    tw.write_all(output.as_bytes()).unwrap();
    tw.flush().unwrap();

    let written = String::from_utf8(tw.into_inner().unwrap()).unwrap();
    println!("{}", written);
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

fn get_common_filetypes(filepath: &str) -> HashMap<String, usize> {
    let paths = glob(&format!("{}/**/*", filepath)).unwrap();

    let mut file_types: HashMap<String, usize> = HashMap::new();

    for path in paths {
        let path = match path {
            Ok(x) => x,
            Err(_) => continue,
        };

        if path.is_dir() {
            continue;
        }

        let extension = match path.extension() {
            Some(ext) => ext.to_str().unwrap().to_string(),
            None => String::from("Unknown"),
        };

        let count = file_types.entry(extension).or_insert(0);
        *count += 1;
    }

    return file_types;
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

        let result = count(test_dir, "test_file2.rs");
        assert_eq!(result, 2);
    }

    #[test]
    fn test_common() {
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

        let result = get_common_filetypes(test_dir);

        assert_eq!(result.values().count(), 2);
        assert_eq!(result.get("txt").unwrap(), &1);
        assert_eq!(result.get("rs").unwrap(), &2);
    }
}
