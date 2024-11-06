#![feature(iter_advance_by)]



use std::rc::Rc;
use std::{env, io::Error, os};

use std::path::PathBuf;
use csv::{Writer, WriterBuilder};
use regex::Regex;


fn parse_file(filename : &PathBuf ) -> (Vec<String>, Vec<String>,Vec<Vec<String>> ) {
    let re = Regex::new(r"\d\d\d\.\d\d").unwrap();
    let file = csv::ReaderBuilder::new()
        .flexible(true)
        .from_path(filename)
        .unwrap();
    let mut records = file.into_records();
    let wlem = records
        .next()
        .unwrap()
        .unwrap()
        .into_iter()
        .map(|head| match re.find(head) {
            Some( wl) => Some(wl.as_str().to_owned()),
            None => None  
        })
        .flatten()
        .collect::<Vec<String>>();
    let mut wlex = vec![];
    let mut ints: Vec<Vec<String>> = vec![];
    records.advance_by(1).unwrap();
    let mut ilprevio = vec![String::from("e")];
    for  (nline, record) in records.enumerate() {
        let inner = record
            .inspect_err(|err| {
                println!("{:?}", err);
                println!("{:?}", ilprevio)
            })
            .unwrap();
        if inner.as_slice().starts_with(|character: char| character.is_alphabetic() ) {
            break;
        }
        let mut row: Vec<String> = inner.into_iter().map(|n|n.to_owned()).collect();
        ilprevio = row.clone();
        let wl: String = row.remove(0);
        ints.push(row);
        wlex.push(wl);
    }
    return (wlem, wlex, ints);
}
fn process_file(pb :PathBuf) { 
    
    match pb.extension().and_then(|s| s.to_str()) {
        Some("csv") | Some("CSV")  => (),
        _ => panic!("Expecting a CSV file"),
    }
    let sinex = pb.file_stem().unwrap();
    let (xx, yy, zz) = parse_file(&pb);
    let _ = Writer::from_path(format!("{}.xx.csv", sinex.to_str().unwrap())).unwrap().write_record(xx).unwrap();
    let _ = Writer::from_path(format!("{}.yy.csv", sinex.to_str().unwrap())).unwrap().write_record(yy).unwrap();
    let mut matrix_writer = csv::Writer::from_path(format!("{}.zz.csv", sinex.to_str().unwrap())).unwrap();
    for record in zz.into_iter() {
        matrix_writer.write_record(record).unwrap();
    }
    
}

fn main() {
    let file = env::args().nth(1);
    match file {
        Some(inner) => {
             let sce = PathBuf::from(inner);
             process_file(sce);
        },
        None => println!("No file provided.\nExpexted usage\neemsc file.csv")
    };
}
