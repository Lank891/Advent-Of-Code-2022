mod utils;
mod filesystem;
use regex::Regex;

use crate::filesystem::GetSize;

fn main() {
    let input_file = utils::input_file_path();
    if let Err(e) = input_file {
        println!("{}", e);
        return;
    }

    let mut root = filesystem::Directory {
        name: String::from("/"),
        files: Vec::new(),
        subdirectories: Vec::new(),
    };

    let mut working_directory : Vec<&mut filesystem::Directory> = Vec::new();
    working_directory.push(&mut root);

    let cd_regex = Regex::new(r"\$ cd (/|\.\.|\w+)").unwrap();
    let ls_regex = Regex::new(r"\$ ls").unwrap();
    let item_ls_regex = Regex::new(r"(\d+) ([\w\.]+)").unwrap();
    let dir_ls_regex = Regex::new(r"dir ([\w\.]+)").unwrap();

    if let Ok(lines) = utils::read_lines(input_file.unwrap()) {
        for line in lines {
            if let Ok(readed_line) = line {
                
                if let Some(cd) = cd_regex.captures(&readed_line) {
                    let subcommand = cd.get(1).unwrap().as_str();
                    
                    if subcommand == "/" {
                        while working_directory.len() > 1 {
                            working_directory.pop();
                        }
                    }
                    else if subcommand == ".." && working_directory.len() > 0{
                        working_directory.pop();
                    }
                    else {
                        unsafe {
                            let dir = *working_directory.last_mut().unwrap() as *mut filesystem::Directory;
                            for i in 0..(*dir).subdirectories.len() {
                                if (*dir).subdirectories[i].name == subcommand {
                                    working_directory.push(&mut (*dir).subdirectories[i]);
                                    break;
                                }
                            }
                        }
                    }

                }
                
                else if let Some(_ls) = ls_regex.captures(&readed_line) {
                    // Do nothing technically
                }

                else if let Some(item_ls) = item_ls_regex.captures(&readed_line) {
                    let size = item_ls.get(1).unwrap().as_str().parse::<u64>().unwrap();
                    let name = item_ls.get(2).unwrap().as_str();
                    
                    (*working_directory.last_mut().unwrap()).files.push(filesystem::File {name: String::from(name), size});
                }

                else if let Some(dir_ls) = dir_ls_regex.captures(&readed_line) {
                    let name = dir_ls.get(1).unwrap().as_str();
                    (*working_directory.last_mut().unwrap()).subdirectories.push(filesystem::Directory {name: String::from(name), files: Vec::new(), subdirectories: Vec::new()});
                }
            }
        }
    }

    let mut sizes = root.get_directory_sizes(None);
    sizes.sort_by(|(_name1, size1), (_name2, size2) | size1.cmp(size2));
    let sum_sizes_part_1 : u64 = sizes.iter().map(|(_name, size)| size).filter(|size| **size <= 100000).sum();
    let minimum_to_remove = 30000000 - (70000000 - root.size());
    //println!("{:#?}", sizes);
    let dir_to_delete_part_2 = sizes.iter().filter(|(_name, size)| *size >= minimum_to_remove).next().unwrap().clone();
    println!("Part 1: {}", sum_sizes_part_1);
    println!("Part 2: {}", dir_to_delete_part_2.1);
}