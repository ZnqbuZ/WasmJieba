use std::{fs, io};
use std::io::{BufRead, BufReader, BufWriter, Seek, Write};
use std::path::Path;
use log::{debug, error, info};

fn new_dir(path: &Path) -> io::Result<()> {
    if path.exists() {
        fs::remove_dir_all(path)?;
    }

    fs::create_dir(path)
}

fn dict_iter(src_dir: &Path, word_len: usize) {
    let dst_dir = src_dir.join(
        format!("../{}.ime_dicts", word_len));
    new_dir(dst_dir.as_path()).unwrap();

    let dst2_dir = src_dir.join(format!("../{}.ime_dicts", word_len + 1));
    new_dir(dst2_dir.as_path()).unwrap();

    let src_dicts = fs::read_dir(src_dir).unwrap();
    for src_dict in src_dicts {
        let src_path = src_dict.as_ref().unwrap().path();
        let src = src_path.as_path();
        let mut src_reader = BufReader::new(
            fs::OpenOptions::new()
                .read(true)
                .open(src)
                .unwrap());

        let dst_path = dst_dir.join(
            src_dict
                .as_ref().unwrap()
                .path()
                .file_name().unwrap()
                .to_str().unwrap());
        let dst = dst_path.as_path();
        let mut dst_writer = BufWriter::new(
            fs::OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(dst)
                .unwrap());

        info!("Writing {:?} -> {:?}.", src, dst);

        for (uidx, line) in (&mut src_reader).lines().enumerate() {
            if line.as_ref().unwrap().chars().count() <= word_len {
                dst_writer.write(
                    format!("{}\n", line.unwrap()).as_bytes()).unwrap();
            }
        }

        src_reader.rewind().unwrap();
        dst_writer.flush().unwrap();

        let mut jieba = jieba_rs::Jieba::new();
        let mut dst_reader = BufReader::new(
            fs::OpenOptions::new()
                .read(true)
                .open(dst)
                .unwrap());

        let dst2_path = dst2_dir.join(
            src_dict
                .as_ref().unwrap()
                .path()
                .file_name().unwrap()
                .to_str().unwrap());
        let dst2 = dst2_path.as_path();
        let mut dst2_writer = BufWriter::new(
            fs::OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(dst2_path.as_path())
                .unwrap());

        info!("Writing {:?} -> {:?}.", src, dst2);
        jieba.load_dict(&mut dst_reader).unwrap();
        for (uidx, line) in (&mut src_reader).lines().enumerate() {
            let word = line.unwrap();
            let words = jieba.cut(&word, false);
            if words.len() >= word.chars().count() {
                debug!("Not cut-able word: {}, from {}, line {}", word, src_path.display(), uidx);
                if word.chars().count() <= word_len {
                    error!("Some thing wrong here: {:?} => {:?}", word, words);
                }
                if word.chars().count() == word_len + 1 {
                    dst2_writer.write(
                        format!("{}\n", word).as_bytes()).unwrap();
                }
            }
        }
    };

    let dst_dicts = fs::read_dir(dst_dir).unwrap();
    for dst_dict in dst_dicts {
        println!("include_dict!(\"{}\");", dst_dict.unwrap().file_name().to_str().unwrap());
    }
}

fn main() -> io::Result<()> {
    env_logger::init();

    let ime_dicts = Path::new("../../../assets/ime_dicts");

    // This function generates 2 groups of ime_dicts:
    //      1. 2-char words;
    //      2. 3-char words that cannot be cut with 1.
    // I found that most words in 2. should not be treated as single words,
    // so I will import only 1. to wasmjieba.
    dict_iter(ime_dicts, 2);

    Ok(())
}
