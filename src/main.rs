#![feature(iter_advance_by)]
use std::iter::Peekable;
use std::{env, vec};
use std::path::PathBuf;
use csv::{StringRecord, Writer, StringRecordsIntoIter};
use regex::Regex;
use std::fs::File;

fn parse_file(filename : &PathBuf) -> (Vec<String>, Vec<String>, Vec<Vec<String>>) {
    let file = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_path(filename)
        .unwrap();
    let mut records = file.into_records().peekable();
    let (xx,yy,zz) = match  records.peek(){
        Some(Ok(record)) if record.as_slice().contains("Wavelength (nm)") => {
            println!("Mètodo pelotudo para el file {}", filename.as_path().file_name().unwrap().to_str().unwrap());
            parse_formato_pelotudo(records)
        },
        Some(Ok(_record)) => {
            println!("Otro método para {}",filename.as_path().file_name().unwrap().to_str().unwrap());
            parse_formato_b(records)
    },
        _ => panic!("No se que pasa con {}", filename.as_path().file_name().unwrap().to_str().unwrap())
    };
    return (xx, yy, zz);
}





fn parse_formato_pelotudo(mut records :  Peekable<StringRecordsIntoIter<File>>) -> (Vec<String>, Vec<String>, Vec<Vec<String>>) {
    records.advance_by(1).unwrap();
    let re = Regex::new(r"\d+\.\d\d").unwrap();
    let wlem = records
        .next()
        .unwrap()
        .unwrap()
        .into_iter()
        .map(|head| match re.find(head) {
            Some( wl) => {
                Some(wl.as_str().to_owned())
            },
            None => None  
        })
        .flatten()
        .collect::<Vec<String>>();
     let mut wlex = vec![];
    let mut ints: Vec<Vec<String>> = vec![];
    records.advance_by(1).unwrap();
    let mut ilprevio = vec![String::from("e")];
    for record in records {
        let inner = record
            .inspect_err(|err| {
                println!("{:?}", err);
                println!("{:?}", ilprevio)
            })
            .unwrap();
            
        if inner.as_slice().starts_with(|character: char| character.is_alphabetic() ) {
            println!("should be here only once");
            break;
        }
        let mut row: Vec<String> = inner.into_iter().map(|n|n.to_owned()).collect();
        ilprevio = row.clone();
        let wl: String = row.remove(0);
        ints.push(row);
        wlex.push(wl);
    }
    println!("Dims:\nxx: {}, yy: {} zz: {} x {}", wlex.len(), wlem.len(), ints.len(),ints.first().unwrap().len());

    return (wlem, wlex, ints);
}



fn parse_formato_b(mut records : Peekable<StringRecordsIntoIter<File>> ) -> (Vec<String>, Vec<String>,Vec<Vec<String>> ) {
    let re = Regex::new(r"\d+\.\d\d").unwrap();
    let wlex = records
        .next()
        .unwrap()
        .unwrap()
        .into_iter()
        .map(|head| match re.find(head) {
            Some( wl) => {
                Some(wl.as_str().to_owned())
            },
            None => None  
        })
        .flatten()
        .collect::<Vec<String>>();
    let mut wlem = vec![];
    let mut ints: Vec<Vec<String>> = vec![];
    'recorditer: for (line, record) in records.enumerate() {
        let mut thisrow = vec![];
        for (col,val) in record.unwrap().into_iter().enumerate() {
            if col.eq(&0) {
                wlem.push(val.to_owned());
                if val.starts_with(|character: char| character.is_alphabetic()) && line.ne(&0) {
                    println!("{}", val);
                    break 'recorditer;
                }
            }
            else if col % 2 == 0 {
                thisrow.push(val.to_owned());
            }
        }
        ints.push(thisrow);
    }
    println!("Dims:\nxx: {}, yy: {} zz: {} x {}", wlex.len(), wlem.len(), ints.len(),ints.first().unwrap().len());

    return (wlex, wlem, ints);
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
        None => println!("No file provided.\nExpected usage:\neemsc file.csv")
    };
}
