use std::{fs, io};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

fn extract_words_2(dicts_path: &Path) {
    let dicts = fs::read_dir(dicts_path).unwrap();
    for dict in dicts {
        let dict_reader = BufReader::new(fs::File::open(
            dict.as_ref().unwrap().path()).unwrap());

        let new_dict_path = dicts_path.join(
            format!("2.{}",
                    dict
                        .as_ref().unwrap()
                        .path()
                        .file_name().unwrap()
                        .to_str().unwrap()));

        let new_dict_file = fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(new_dict_path.as_path())
            .unwrap();

        let mut new_dict_writer = BufWriter::new(new_dict_file);

        println!("Writing {:?} -> {:?}", dict.as_ref().unwrap().path(), new_dict_path.as_path());

        for line in dict_reader.lines() {
            if line.as_ref().unwrap().len() <= 2 {
                new_dict_writer.write(
                    format!("{}\n", line.unwrap()).as_bytes()).unwrap();
            }
        }
    }
}

fn main() -> io::Result<()> {
    env_logger::init();

    extract_words_2(Path::new("examples/build-dict/dicts"));
    
    println!("{:?}", wasmjieba::cut("我们中出了一个叛徒", true));
    Ok(())
}
