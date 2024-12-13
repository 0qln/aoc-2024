#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]
#![feature(iterator_try_reduce)]
#![feature(hash_set_entry)]
#![feature(linked_list_cursors)]

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;

macro_rules! include_using_path {
    ($relative_path:expr) => {{
        use std::path::PathBuf;
        let path = {
            let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            path.push(file!());
            path.pop();
            path.push($relative_path);
            path
        };
        let file_path = path.to_str().unwrap();
        &std::fs::read_to_string(file_path).expect(&format!("Failed to read file: {}", file_path))
    }};
}

pub(crate) use include_using_path;
