use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor};
use std::path::Path;
use std::{env, fs};
use zstd::stream::encode_all;

fn main() {
    let include_dict = HashSet::from(["数学科学.txt", "物理科学.txt", "计算机业.txt"]);

    let exclude_dict = HashSet::from([
        "休闲活动.txt",
        "市场购物.txt",
        "体育运动.txt",
        "人文政治.txt",
        "网络用语.txt",
        "地产开发.txt",
        "敏感用词.txt",
        "美容美发.txt",
        "音乐歌曲.txt",
        "家居装饰.txt",
        "办公文教.txt",
    ]);

    let threshold = 1024 * 1024;

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("dicts.bin");

    println!("cargo:warning=Writing to: {}", dest_path.to_str().unwrap());

    let dict_dir_path = Path::new(&manifest_dir).join("assets/DomainWordsDict/data");
    let dict_list = fs::read_dir(dict_dir_path).unwrap();
    let mut dict_content = String::new();
    for dict in dict_list {
        // continue;
        let dict_name = dict
            .as_ref()
            .unwrap()
            .file_name()
            .to_str()
            .unwrap()
            .to_string();
        if !exclude_dict.contains(dict_name.as_str())
            && (dict.as_ref().unwrap().metadata().unwrap().len() < threshold
                || include_dict.contains(dict_name.as_str()))
        {
            println!("cargo:warning=Including: {}", dict_name);

            for line in BufReader::new(File::open(dict.as_ref().unwrap().path()).unwrap()).lines() {
                let line = line.unwrap();
                let entry: Vec<&str> = line.split_whitespace().collect();
                if entry.len() != 2 || entry[1].parse::<u32>().is_err() {
                    println!("cargo:warning=Invalid entry: {}", line);
                    continue;
                }
                dict_content.push_str(line.as_str());
                dict_content.push_str("\n");
            }
        }
    }

    dict_content.push_str(
        fs::read_to_string(
            Path::new(&manifest_dir)
                .join("assets/dict.txt.big")
                .to_str()
                .unwrap(),
        )
        .unwrap()
        .as_str(),
    );
    dict_content.push_str("\n");

    fs::write(
        &dest_path,
        encode_all(Cursor::new(dict_content), 22).unwrap(),
    )
    .unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets");
}
