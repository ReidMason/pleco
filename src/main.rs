mod args;

use std::{
    collections::HashMap,
    io::Write,
    path::{Path, PathBuf},
};

use args::{Command, PlecoArgs};
use clap::Parser;

extern crate glob;
use glob::glob;
use mime_guess::mime;
use tabwriter::TabWriter;

const EXCLUDED_FILETYPES: [&str; 1] = [".DS_Store"];

fn main() {
    let args = PlecoArgs::parse();

    match args.command {
        Command::ListCommon(x) => {
            let file_types = get_common_filetypes(&x.filepath);
            let file_types = order_file_types(file_types);
            let file_types = format_file_types(file_types);

            println!("Common file types found:");

            print_columns(&file_types);
        }
        Command::PullFiles(x) => {
            pull_files(&x.filepath, &x.output_dir);
        }
        Command::Count(x) => {
            count(&x.filepath, &x.search);
        }
    };
}

fn pull_files(filepath: &str, output_dir: &str) {
    let paths = glob(&format!("{}/**/*", filepath)).unwrap();

    for path in paths {
        let path = match path {
            Ok(x) => x,
            Err(_) => continue,
        };

        if path.is_dir() {
            continue;
        }

        if let Some(filename) = path.file_name() {
            if EXCLUDED_FILETYPES.contains(&filename.to_str().unwrap()) {
                continue;
            }
        }

        let kind = mime_guess::from_path(&path)
            .first()
            .unwrap_or(mime::TEXT_PLAIN);

        if !match kind.type_() {
            mime::IMAGE => true,
            mime::VIDEO => true,
            _ => false,
        } {
            continue;
        }

        let new_filepath = get_new_filepath(&path, output_dir);
        std::fs::create_dir_all(new_filepath.parent().unwrap()).unwrap();
        std::fs::copy(&path, &new_filepath).unwrap();
    }
}

fn get_new_filepath(path: &PathBuf, output_dir: &str) -> PathBuf {
    let base = Path::new(output_dir);
    let new_filepath = match path.parent() {
        Some(parent) => match parent.file_name() {
            Some(parent_filename) => match parent.parent() {
                Some(gp) => match gp.file_name() {
                    Some(gp_filename) => base.join(gp_filename).join(parent_filename),
                    None => base.join(parent_filename),
                },
                None => base.join("root"),
            },
            None => base.join("root"),
        },
        None => base.join("root"),
    };

    return new_filepath.join(path.file_name().unwrap());
}

fn format_file_types(file_types: Vec<(String, usize)>) -> String {
    file_types
        .into_iter()
        .map(|(file_type, count)| format!("{}\t{}", file_type, count))
        .collect::<Vec<String>>()
        .join("\n")
}

fn order_file_types(file_types: HashMap<String, usize>) -> Vec<(String, usize)> {
    let mut file_types_vec: Vec<(String, usize)> = file_types.into_iter().collect();
    file_types_vec.sort_by(|a, b| b.1.cmp(&a.1));
    return file_types_vec;
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

    #[test]
    fn test_get_new_filepath() {
        let output_dir = "output";
        let test_files = vec![
            ("test_dir/test_file1.txt", "output/test_dir/test_file1.txt"),
            ("test_dir/test_file2.rs", "output/test_dir/test_file2.rs"),
            (
                "test_dir/nested/test_file2.rs",
                "output/test_dir/nested/test_file2.rs",
            ),
            (
                "top_dir/test_dir/nested/test_file2.rs",
                "output/test_dir/nested/test_file2.rs",
            ),
            ("file.rs", "output/root/file.rs"),
        ];

        for tc in test_files.iter() {
            let path = Path::new(tc.0).to_path_buf();
            let result = get_new_filepath(&path, output_dir);

            assert_eq!(result.to_str().unwrap(), tc.1);
        }
    }

    #[test]
    fn test_pull_files() {
        let dir = tempdir().unwrap();

        let test_files = vec![
            ("test_dir/test_file1.txt", "test_dir/test_file1.txt"),
            ("test_dir/test_file2.rs", "test_dir/test_file2.rs"),
            (
                "test_dir/nested/test_file2.rs",
                "test_dir/nested/test_file2.rs",
            ),
            (
                "top_dir/test_dir/nested/test_file2.rs",
                "test_dir/test_file2.rs",
            ),
            ("file.rs", "root/file.rs"),
        ];

        let test_dir = dir.path().to_str().unwrap();
        for tc in test_files.iter() {
            let file_path = dir.path().join(tc.0);
            let prefix = file_path.parent().unwrap();
            std::fs::create_dir_all(prefix).unwrap();
            File::create(file_path).unwrap();
        }

        // pull_files(test_dir);
    }
}
