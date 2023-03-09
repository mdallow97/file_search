use filesize::PathExt;
use std::{
    fs,
    path::{Path, PathBuf},
};

// TODO: this function almost exactly mimics list_files,
//       there is some optimazation to be found
fn dir_size(path: &Path) -> u64 {
    let mut dirs: Vec<PathBuf> = Vec::new();
    let mut size: u64 = 0;

    let cur_dir = match fs::read_dir(path) {
        Ok(dir) => dir,
        Err(_) => {
            println!("{}: Cannot open dir!", path.display());
            return 0;
        }
    };

    for entry in cur_dir {
        let entry_path = entry.unwrap().path();

        let metadata = match fs::metadata(&entry_path) {
            Ok(data) => data,
            Err(_) => {
                println!("{}: Denied access to metadata!", entry_path.display());
                continue;
            }
        };

        if metadata.is_file() {
            let filesize = match entry_path.size_on_disk() {
                Ok(size) => size,
                Err(_) => {
                    println!("{}: Cannot determine file size!", entry_path.display());
                    continue;
                }
            };
            size += filesize;
        } else if metadata.is_dir() {
            dirs.push(entry_path.clone());
        } else {
            println!("{}: Entry type unknown!", entry_path.display());
        }
    }

    for dir in dirs {
        size += dir_size(&dir);
    }

    return size;
}

fn list_files(path: &Path, depth: Option<u32>) -> Vec<(PathBuf, u64)> {
    let mut dirs: Vec<PathBuf> = Vec::new();
    let mut file_and_sizes: Vec<(PathBuf, u64)> = Vec::new();

    let cur_dir = match fs::read_dir(path) {
        Ok(dir) => dir,
        Err(_) => {
            println!("{}: Cannot open dir!", path.display());
            return vec![];
        }
    };

    for entry in cur_dir {
        let entry_path = entry.unwrap().path();

        let metadata = match fs::metadata(&entry_path) {
            Ok(data) => data,
            Err(_) => {
                println!("{}: Denied access to metadata!", entry_path.display());
                continue;
            }
        };

        if metadata.is_file() {
            let filesize = match entry_path.size_on_disk() {
                Ok(size) => size,
                Err(_) => {
                    println!("{}: Cannot determine file size!", entry_path.display());
                    continue;
                }
            };
            file_and_sizes.push((entry_path, filesize));
        } else if metadata.is_dir() {
            dirs.push(entry_path.clone());
        } else {
            println!("{}: Entry type unknown!", entry_path.display());
        }
    }

    if let Some(n) = depth {
        if n > 0 {
            for dir in dirs {
                file_and_sizes.append(&mut list_files(&dir, Some(n - 1)));
            }
        }
    } else {
        for dir in dirs {
            file_and_sizes.append(&mut list_files(&dir, None));
        }
    }

    return file_and_sizes;
}

fn find_n_largest_files(mut files: Vec<(PathBuf, u64)>, n: usize) -> Vec<(PathBuf, u64)> {
    files.sort_by(|a, b| b.1.cmp(&a.1));

    if files.len() <= n {
        return files.clone();
    } else {
        return files[..n].to_vec();
    }
}

// TODO: write a unit test for this
fn bytes_size_to_str(size: u64) -> String {
    let size_map = Vec::from([
        ((10 as u64).pow(0), "B"),
        ((10 as u64).pow(3), "KB"),
        ((10 as u64).pow(6), "MB"),
        ((10 as u64).pow(9), "GB"),
        ((10 as u64).pow(12), "TB"),
        ((10 as u64).pow(15), "PB"),
    ]);

    let mut last_x = size as f64;
    // FIXME: feels like there should be a more efficient way than to clone below
    for (i, (exp, s)) in size_map.clone().into_iter().enumerate() {
        let x = (size as f64) / (exp as f64);

        if x <= 1.0 {
            if i == 0 {
                return format!("{:.2} {}", last_x, s.to_string());
            } else {
                return format!("{:.2} {}", last_x, size_map[i - 1].1.to_string());
            }
        }
        last_x = x;
    }

    return format!(
        "{:.2} {}",
        last_x,
        size_map[size_map.len() - 1].1.to_string()
    );
}

fn main() {
    let path = Path::new("C:\\");

    let size = dir_size(path);
    println!(
        "Path {} has size {}",
        path.display(),
        bytes_size_to_str(size)
    );

    let file_list = list_files(path, None);
    let largest = find_n_largest_files(file_list, 10);
    println!("Largest files: {:?}", largest);
}
