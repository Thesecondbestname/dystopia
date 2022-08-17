use crate::io::BufReader;
use std::io::BufRead;
use std::io::{Read,Write};
use std::fs::*;
use std::fs::{read_to_string, write};
use std::io;

#[derive(Debug)]

struct Options {
    prompt: String,
    options: Vec<String>,
}
fn main() {
    if !std::path::Path::new("games.txt").exists() {
        File::create("games.txt").expect("failed to create games.txt");
    }
    let path = match File::open(&game_selection()){
        Ok(n) => n,
        Err(..)=> File::open(get_and_write_path("Datei nicht gefunden! Gib den pfad zu einer verfügbaren Datei ein.")).unwrap(),
    };
    let mut location = 1;
    println!("{:#?}",parser(path,location));
}
fn io_handler() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(..) => {}
        Err(error) => println!("error: {error}"),
    }
    return input;
}
fn matcher(options: &Vec<String>, prompt: &String) -> String {
    println!("{}", prompt);
    for option in options.iter() {
        println!("{}", option);
    }
    let input = &io_handler();
    return input.to_string();
}
fn parser(file: File, location: i32) -> Options {
    let reader = BufReader::new(file);
    let mut optionen: Vec<String> = vec![];
    let mut prompt = String::new();
    let mut found_prompt = false;
    let mut right_identification = false;
    for (_index, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(n) => n,
            Err(e) => {
                println!(
                    "Oh smokes! Dieses passierte : {}\n\n Mal gucken was passiert ;)",
                    e
                );
                continue;
            }
        };
        if &line == "" {
            continue;
        }

        if !found_prompt {
            right_identification =
                line[0..location.to_string().len()].contains(&location.to_string());
        }
        if right_identification {
            if !found_prompt {
                prompt = line[location.to_string().trim().len()..].to_string();
                found_prompt = true;
                continue;
            }
            if line.chars().collect::<Vec<char>>().last().unwrap() == &';' {
                optionen.push(line.to_string());
            } else {
                if optionen != [""] {
                    break;
                }
            }
        }
    }
    return Options {
        prompt: prompt,
        options: optionen,
    };
}
fn game_selection() -> String {
    let lines = read_to_string("games.txt").expect("Konnte die 'games.txt' Datei nicht finden oder lesen. Stell sicher, dass die Datei nicht zu groß ist.");
    let mut open_games: Vec<String> = lines.split("\n").map(|x| x.to_string()).collect();
    open_games.push("neues Spiel".to_string());
    open_games.retain(|x| x != "");
    if lines == "" {
        return get_and_write_path("Gib den Pfad zum ersten Spiel ein!");
    };
    let input: i32 = prompt_int(
        &open_games
            .iter()
            .map(|opt| {
                if opt != "" {
                    format!("- {}", opt)
                } else {
                    "------------".to_string()
                }
            })
            .collect(),
        "Such dir ein Spiel aus oder fang ein neues an\n".to_string(),
    )  - 1;
    if input == open_games.len().try_into().unwrap() {
        return get_and_write_path("Gib den Pfad zur Spiel Datei ein:");
    }
    return open_games[input as usize].to_string();
}
fn prompt_int(options: &Vec<String>, mut prompt: String) -> i32 {
    loop {
        match matcher(options, &prompt).trim().parse() {
            Ok(n) => {
                return n;
            }
            Err(e) => {
                prompt = "Keine Gültige Zahl eingegeben! Versuchs nochmal.".to_string();
                println!("{}", e);
            }
        };
    }
}
fn get_and_write_path(prompt: &str) -> String {
    loop{
        println!("{}", prompt);
        let path = io_handler();
        if !std::path::Path::new("games.txt").exists(){
            println!("Konnte die angegebene Datei nicht finden.");
            continue
        }
        println!("So?: {} [N,y]",path);
        match io_handler().as_str(){
            "Y"|"y" => () ,
            "N"|"n" => continue,
            _ => continue,
        }
        println!("I Waas hre");
        match write("games.txt", format!("{:#?}{}",read(String::from("games.txt")),&path)) {
            Ok(n) => n,
            Err(..) => {
                println!("Konnte nicht in die games.txt Datei schreiben.")
            }
        }
    break path;
    }
}
fn read(file: String) -> BufReader<File>{
    let read = match File::open(&file){
        Ok(n) => n,
        Err(e) => {println!("{} konnte die Datei {} nicht lesen\n",e,file); panic!()},
    };
    return BufReader::new(read);
}