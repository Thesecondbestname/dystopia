use std::fmt::Display;
use std::fs::OpenOptions;
use std::fs;
use crate::io::BufReader;
use std::path::{Path, self};
use std::io::{BufRead, Read};
use std::io::{Write};
use std::fs::*;
use std::fs::{read_to_string};
use std::io;

struct Options {
    prompt: String,
    options: Vec<option>,
}

enum Color{
    red,
    green,
    blue,
}
enum Action{
    mashKey,
    mashMouse,
    terminal,
    color,
    clear,
    lose,
    null,
}
struct Option{
    option_str: String,
    point_loc: i32,
    action1 : Action,
    action2 : Action,
    action3 : Action,
    action4 : Action
}
fn main() {
    if !Path::new("games.txt").exists() {
        File::create("games.txt").expect("failed to create games.txt");
    }
    let path = &game_selection()[..];
    println!("{}",path);
    let file = match File::open(path){
        Ok(n) => n,
        Err(..) => {println!("Datei nicht gefunden!");File::open(&get_and_write_path("Gib den pfad zu einer verfügbaren Datei ein.")[..]).unwrap()}
    };
    let mut location = locationeer(true,&file);
    println!("{}",location);
    loop{
        game_handler(path, location);
    }

}
fn io_handler() -> String {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(..) => {}
        Err(error) => println!("error: {error}"),
    }
    return input;
}
fn matcher(optionen: &Options) -> String {
    println!("{}", optionen.prompt);
    for option in optionen.options.iter() {
        println!("{}", option);
    }
    let input = &io_handler();
    return input.to_string();
}
fn parser(reader: BufReader<&File>, location: i32) -> Options {
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
    println!("done!");
    return analyze(prompt, optionen);

}
fn game_selection() ->  String {
    let lines = read_to_string("games.txt").expect("Konnte die 'games.txt' Datei nicht finden oder lesen. Stell sicher, dass die Datei nicht zu groß ist.");
    let mut open_games: Vec<String> = lines.split("\n").map(|x| x.to_string()).collect();
    open_games.push("neues Spiel".to_string());
    open_games.retain(|x| x != "");
    if lines == "" {
        return get_and_write_path("Gib den Pfad zum ersten Spiel ein!");
    };
    
    let input: i32 = prompt_int(Options{options:
        open_games
            .iter()
            .map(|opt| {
                if opt != "" {
                    format!("- {}", opt)
                } else {
                    "------------".to_string()
                }
            })
            .collect(),
        prompt: "Such dir ein Spiel aus oder fang ein neues an\n".to_string()},
    )  - 1;
    if input +1 == open_games.len().try_into().unwrap() {
        return get_and_write_path("Gib den Pfad zur Spiel Datei ein:");
    }
    return open_games[input as usize].to_string();
}
fn prompt_int(mut optionen: Options) -> i32 {
    loop {
        match matcher(&optionen).trim().parse() {
            Ok(n) => {
                return n;
            }
            Err(e) => {
                optionen.prompt = "Keine Gültige Zahl eingegeben! Versuchs nochmal.".to_string();
            }
        };
    }
}
fn get_and_write_path(prompt: &str) -> String {
    loop{
        println!("{}", prompt);
        let path_to_file = io_handler();
        println!("{}",path_to_file);
        if !fs::metadata(Path::new(&path_to_file[..])).is_ok(){
            println!("Konnte die angegebene Datei nicht finden.\n");
            continue
        }
        println!("So?: {} [N,y]",path_to_file);
        match io_handler().chars().next(){
            Some('Y')|Some('y') => () ,
            Some('N')|Some('n') => continue,
            _ => continue,
        }
        match append("games.txt".to_string(),&path_to_file) {
            Ok(n) => n,
            Err(..) => {
                println!("Konnte nicht in die games.txt Datei schreiben.")
            }
        }

    break path_to_file;
    }
}
fn append(file: String,input: &String) -> Result<(),std::io::Error>{
    let mut fileRef = OpenOptions::new().append(true).open(file).expect("Unable to open file");
    fileRef.write_all(input.as_bytes())
}
fn game_handler(path: &str, location: i32){
    let file = match File::open(path){
        Ok(n) => n,
        Err(..)=> {println!("Datei nicht gefunden!");File::open(&get_and_write_path("Gib den pfad zu einer verfügbaren Datei ein.")[..]).expect("Noome knows")}
    };
    let reader = BufReader::new(&file);
    /*let raw = &parser(reader,location);
    let pretty = analyze(*raw);
    let pretty = Options{prompt: pretty.prompt, options: pretty.options};
    */matcher(&parser(reader, location));

}
fn locationeer(read: bool, file: &File) -> i32{
    let reader = BufReader::new(file);
    let location = reader.lines().next().unwrap().unwrap();
    match location.trim().parse() {
        Ok(n) => 
            {println!("{}",n);n},
        
        Err(..) => 0
        
    }


}
fn analyze(prompt: String,opts: Vec<String> ) -> Options{
    let prompt = prompt.split(";").next().expect("Datei falsch formatiert. Fehler beim Lesen eines Prompts.");
    let pretty_string = vec![];
    let mut parts = vec![];
    let options_analized  = vec![];
    optionen_vec = vec![];
    for option in opts{
        parts = option.split("//").collect();
        optionen_vec.push(Option{ option_str: parts[1].to_string(), point_loc: parts[2].trim().parse().unwrap())}
    };
    return Options { prompt: (prompt.to_string()), options: () }
}