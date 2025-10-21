use clap::Parser;
use data_enc::{cil, enc};
use indicatif::{MultiProgress, ProgressBar};
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
    let cil = cil::Cil::parse();
    // println!("{:#?}", cil);

    // let cil = cil::Cil {
    //     files: vec![
    //         "/Volumes/LpMacEXT/摄影".to_string(),
    //         "/Volumes/LpMacEXT/color.jpeg".to_string(),
    //     ],
    //     passwd: "2002".to_string(),
    //     mode: cil::EncDecMode::Enc,
    // };

    // let cil = cil::Cil {
    //     files: vec![
    //         // "/Volumes/LpMacEXT/摄影".to_string(),
    //         "/Volumes/LpMacEXT/color.jpeg.enc".to_string(),
    //     ],
    //     passwd: "2002".to_string(),
    //     mode: cil::EncDecMode::Dec,
    // };

    let mut paths: Vec<std::path::PathBuf> = Vec::new();
    cil.files.iter().for_each(|f_path| {
        let path = Path::new(f_path);
        if !path.try_exists().unwrap_or_else(|e| panic!("{}", e)) {
            panic!("路径:{}不存在", f_path);
        }

        if path.is_file() {
            paths.push(path.to_path_buf());
        } else if path.is_dir() {
            WalkDir::new(path).into_iter().for_each(|entry| {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_file() {
                    paths.push(path.to_path_buf());
                }
            });
        }
    });

    paths.iter().for_each(|p| {
        println!("{:?}", p);
    });

    match cil.mode {
        cil::EncDecMode::Enc => enc_files(&paths, &cil.passwd),
        cil::EncDecMode::Dec => dec_files(&paths, &cil.passwd),
    }
}

fn enc_files(paths: &[PathBuf], passwd: &str) {
    let m = MultiProgress::new();
    let pb = m.add(ProgressBar::new(paths.len() as u64));
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap(),
    );

    paths.par_iter().for_each(|p| {
        let data: Vec<u8> = std::fs::read(p).unwrap();
        let enc_data: Vec<u8> = enc::encrypt(&data, passwd).unwrap();
        let enc_path =
            p.with_file_name(format!("{}.enc", p.file_name().unwrap().to_str().unwrap()));
        std::fs::write(enc_path, enc_data).unwrap();
        std::fs::remove_file(p).unwrap();

        pb.inc(1);
    });
}

fn dec_files(paths: &[PathBuf], passwd: &str) {
    let m = MultiProgress::new();
    let pb = m.add(ProgressBar::new(paths.len() as u64));
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap(),
    );

    paths.par_iter().for_each(|p| {
        if let Some(ext) = p.extension() {
            if ext == "enc" {
                let data = std::fs::read(p).unwrap();
                let dec_data = enc::decrypt(&data, passwd).unwrap();
                let dec_path = p.with_extension("");
                std::fs::write(dec_path, &dec_data).unwrap();
                std::fs::remove_file(p).unwrap();
            }
        }

        pb.inc(1);
    });
}
