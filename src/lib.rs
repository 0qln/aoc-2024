#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]
#![feature(iterator_try_reduce)]

mod day_1;
mod day_2;

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
