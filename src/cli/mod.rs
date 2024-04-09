use std::{env::args, io::Read, fmt::Debug};

use loteria_engine::engine::*;

use crate::utils::*;
use anyhow::{Result, anyhow};

/*
pub struct BlackList(pub u8);
pub struct Set(pub usize, pub usize, pub u8);
pub struct MarkPair(pub usize, pub usize, pub usize, pub usize);
pub struct RandomMarkPair;
pub struct RandomCenterMarkPair;
pub struct UpperCenterMarkPair;
pub struct LowerCenterMarkPair;
 */

trait ActDebug : BoardActor + Debug {}

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


fn get_instructions<S: AsRef<str>>(lines : &[S]) -> Vec<Box<dyn ActDebug>>{
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


fn c_instructions() -> Result<Vec<Box<dyn ActDebug>>>{
    let mut file = std::fs::File::open(get_instruction_path()?).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let text = String::from_utf8(buffer).unwrap();
    let lines : Vec<_> = text.lines().collect();

    Ok(get_instructions(&lines))
}

pub fn run() -> Result<Vec<Board>> {
    let mut args : Vec<_> = args().collect();
    args.remove(0);
    let instructions = if args.len() == 0{
        c_instructions()?
    }
    else{
        get_instructions(&args)
    };

    if instructions.len() == 0{
        println!("no instructions found");
        return Ok(Vec::new());
    }

    for i in &instructions{
        println!("{:?}", i);
    }

    let mut board = BoardBuilder::new();
    for instruction in instructions{
        instruction.act_on(&mut board)?;
    }
    Ok(board.generate_tape().generate_boards())
}
