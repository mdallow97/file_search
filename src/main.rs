use std::{path::{Path, PathBuf}, fs};
use filesize::PathExt;



fn list_files(path: &Path, depth: Option<u32>) -> Vec<(PathBuf, u64)> {
    let mut dirs: Vec<PathBuf> = Vec::new();
    let mut file_and_sizes: Vec<(PathBuf, u64)> = Vec::new();

    let cur_dir = match fs::read_dir(path) {
        Ok(dir) => dir,
        Err(_) => {
            println!("{}: Cannot open dir!", path.display());
            return vec![]
        }
    };

    for entry in cur_dir {
        let entry_path = entry.unwrap().path();
        
        let metadata = match fs::metadata(&entry_path) {
            Ok(data) => data,
            Err(_) => {
                println!("{}: Denied access to metadata!", entry_path.display());
                continue
            }
        };

        
        if metadata.is_file() {
            let filesize = match entry_path.size_on_disk() {
                Ok(size) => size,
                Err(_) => {
                    println!("{}: Cannot determine file size!", entry_path.display());
                    continue
                },
            };
            file_and_sizes.push((entry_path, filesize));
            // println!("{}, {}", entry_path.display(), filesize);
        }
        else if metadata.is_dir() {
            dirs.push(entry_path.clone());
        }
        else {
            println!("{}: Entry type unknown!", entry_path.display());
        }
    }

    if let Some(n) = depth {
        if n > 0 {
            for dir in dirs {
                file_and_sizes.append(&mut list_files(&dir, Some(n-1)));
            }
        }
    }
    else {
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
    }
    else {
        return files[..n].to_vec();
    }
}

fn main() {
    let path = Path::new("C:\\Users\\");

    let file_list = list_files(path, Some(4));
    let largest = find_n_largest_files(file_list, 3);
    println!("Largest files: {:?}", largest);
}
