use std::{fmt::Debug, io::Read, path::PathBuf};

use loteria_engine::engine::*;
use anyhow::{Result, anyhow};

pub trait ActDebug : BoardActor + Debug {}

impl ActDebug for BlackList{}
impl ActDebug for Set {}
impl ActDebug for MarkPair {}
impl ActDebug for RandomMarkPair {}
impl ActDebug for RandomCenterMarkPair {}
impl ActDebug for UpperCenterMarkPair {}
impl ActDebug for LowerCenterMarkPair {}
impl ActDebug for SetCount {}
impl ActDebug for SetTotal {}
impl ActDebug for SetPair {}

fn str_to_ins(s: &str) -> Option<Box<dyn ActDebug>>{
    if s == "RandomMarkPair"{ Some(Box::new(RandomMarkPair::new())) }
    else if s == "RandomCenterMarkPair" { Some(Box::new(RandomCenterMarkPair::new()))}
    else if s == "UpperCenterMarkPair" { Some(Box::new(UpperCenterMarkPair::new())) }
    else if s == "LowerCenterMarkPair" {Some(Box::new(LowerCenterMarkPair::new())) }
    else{
        let divd : Vec<_> = s.split(" ").collect();
        if divd[0] == "BlackList" {
            Some(Box::new(BlackList::new(u8::from_str_radix(divd[1], 10).unwrap())))
        }
        else
        if divd[0] == "Set" {
            Some(Box::new(Set::new(
                        usize::from_str_radix(divd[1], 10).unwrap(),
                        usize::from_str_radix(divd[2], 10).unwrap(),
                        u8::from_str_radix(divd[3], 10).unwrap()
                        )))
        }
        else
        if divd[0] == "SetTotal" {
            Some(Box::new(SetTotal::new(
                        usize::from_str_radix(divd[1], 10).unwrap(),
                        )))
        }
        else
        if divd[0] == "SetCount" {
            Some(Box::new(SetCount::new(
                        usize::from_str_radix(divd[1], 10).unwrap(),
                        )))
        }
        else
        if divd[0] == "MarkPair" {
            Some(Box::new(MarkPair::new(
                        usize::from_str_radix(divd[1], 10).unwrap(),
                        usize::from_str_radix(divd[2], 10).unwrap(),
                        usize::from_str_radix(divd[3], 10).unwrap(),
                        usize::from_str_radix(divd[4], 10).unwrap()
                        )))
        }
        else
        if divd[0] == "SetPair" {
            Some(Box::new(SetPair::new(
                        usize::from_str_radix(divd[1], 10).unwrap(),
                        usize::from_str_radix(divd[2], 10).unwrap(),
                        )))
        }
        else{
            None
        }
    }
}


pub fn get_instructions<S: AsRef<str>>(lines : &[S]) -> Vec<Box<dyn ActDebug>>{
    let mut v = Vec::new();

    for line in lines{
        let q = str_to_ins(line.as_ref());
        match q {
            Some(i) => v.push(i),
            _ => {},
        }
    }

    v
}

pub fn c_instructions(path: PathBuf) -> Result<Vec<Box<dyn ActDebug>>>{
    let mut file = std::fs::File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let text = String::from_utf8(buffer)?;
    let lines : Vec<_> = text.lines().collect();

    Ok(get_instructions(&lines))
}

pub fn run(inst: Vec<Box<dyn ActDebug>>) -> Result<Vec<Board>> {
    if inst.len() == 0{
        return Err(anyhow!("no instructions where provided"));
    }

    for i in &inst{
        println!("{:?}", i);
    }

    let mut board = BoardBuilder::new();
    for instruction in inst{
        instruction.act_on(&mut board)?;
    }
    Ok(board.generate_tape().generate_boards())
}
