use ffprobe::ffprobe;
use std::error::Error;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use crate::error::ConvertErr;

fn convert(path: &PathBuf, out_path: &PathBuf) -> Result<(), ConvertErr> {

    let args= "-c:v libsvtav1 -crf 35 -preset 6 -svtav1-params tune=0 -g 240 -c:a libopus -b:a 128k -ac 2 -c:s copy";
    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(path.to_str().unwrap())
        .args(args.split(" "))
        .arg(out_path.to_str().unwrap())
        .output().expect("ffmpeg Failed");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    if let Some(ret) = output.status.code(){
        if ret == 0{
            Ok(())
        } else {
            Err(ConvertErr::FfmpegErr)
        }
    }else{
        Err(ConvertErr::ProcError)
    }

    
}

fn remux(path: &PathBuf, out_path: &PathBuf) -> Result<(), ConvertErr> {
    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(path.to_str().unwrap())
        .arg("-c:v")
        .arg("copy")
        .arg(out_path.to_str().unwrap())
        .output().expect("ffmpeg Failed");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    if let Some(ret) = output.status.code(){
        if ret == 0{
            Ok(())
        } else {
            Err(ConvertErr::FfmpegErr)
        }
    }else{
        Err(ConvertErr::ProcError)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ProcessType<'a> {
    Remux(&'a PathBuf),
    Convert(&'a PathBuf),
    Nop,
}

fn filter_files(path: &PathBuf) -> Result<ProcessType, Box<dyn Error>> {
    let mut is_av1 = false;

    if let Ok(probe) = ffprobe(path) {
        for stream in probe.streams {
            if stream.codec_name.unwrap().eq("av1") {
                is_av1 = true;
                break;
            }
        }
    }

    if is_av1 {
        if path.extension().unwrap() != "webm" {
            Ok(ProcessType::Remux(path))
        } else {
            Ok(ProcessType::Nop)
        }
    } else {
        Ok(ProcessType::Convert(path))
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_sort() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_files/");

        let mut convert_path = d.clone();
        convert_path.push("convert.mkv");
        let files = filter_files(&convert_path).unwrap();
        println!("{:?}", files);
        assert_eq!(ProcessType::Convert(&convert_path), files);

        let mut remux_path = d.clone();
        remux_path.push("remux.mkv");
        let files = filter_files(&remux_path).unwrap();
        println!("{:?}", files);
        assert_eq!(ProcessType::Remux(&remux_path), files);

        let mut remux_path = d.clone();
        remux_path.push("remux.webm");
        let files = filter_files(&remux_path).unwrap();
        println!("{:?}", files);
        assert_eq!(ProcessType::Nop, files);
    }

    #[test]
    fn test_mux() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_files/");

        let mut convert_path = d.clone();
        let mut out_path = d.clone();
        convert_path.push("remux.mkv");
        out_path.push("out1.webm");
        let _files = remux(&convert_path, &out_path);
        assert!(out_path.exists());

        let _ = fs::remove_file(out_path);
        assert!(_files.is_ok())
    }

    #[test]
    fn test_convert() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_files/");

        let mut convert_path = d.clone();
        let mut out_path = d.clone();
        out_path.push("out2.webm");
        convert_path.push("convert.mkv");
        let _files = convert(&convert_path, &out_path);

        assert!(out_path.exists());

        let _ = fs::remove_file(out_path);

        assert!(_files.is_ok())
    }
}
