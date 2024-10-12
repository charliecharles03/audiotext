use core::str;
use std::process::{Command, Output};
use std::env;

//ffmpeg -i output_file.mov -t 20 -acodec pcm_s16le -ar 16000 -ac 1 audio_stream.wav
pub fn ffmpeg_part(path : String,output_file_name : String){ 
     let mp= path.as_str();
     let ofn = output_file_name.as_str();
     Command::new("ffmpeg")
        .args(["-i",mp,"-t","20","-acodec","pcm_s16le","-ar","16000","-ac","1",ofn])
        .status()
        .expect("failed to execute process");
         println!("{:?}","we went all the way through"); 
}

// comment is all i need to real i can't really seem to enjoy this font
pub fn whisper_part(){
    let path = "/mnt/c/Users/charlie/Documents/CLI_tools/rustcpp/audio_stream.wav";
                                                                                   
    let path_to_model = "../whisper.cpp/models/ggml-base.en.bin"; //env variable
    Command::new("../whisper.cpp/main")
        .args(["-m",path_to_model,"-otxt","-f",path])
        .status()
        .expect("getting help from whispher failed");
}

fn main() {
    let input_file_name = String::from("2024-05-28 01-44-50.mp4"); // this should be from user
    let output_file_name = String::from("audio_stream.wav");
    ffmpeg_part(input_file_name,output_file_name); //pot change of leak 
    whisper_part(); 
}



