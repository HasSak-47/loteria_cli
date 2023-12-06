use std::{env::args, io::Read, fmt::Debug};

use loteria_engine::engine::*;

use crate::{
    error::LoteriaResult,
    utils::*
};

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

impl ActDebug for BlackList {}
impl ActDebug for Set {}
impl ActDebug for MarkPair {}
impl ActDebug for RandomMarkPair {}
impl ActDebug for RandomCenterMarkPair {}
impl ActDebug for UpperCenterMarkPair {}
impl ActDebug for LowerCenterMarkPair {}

fn str_to_ins(s: &str) -> Option<Box<dyn ActDebug>>{
    if s == "RandomMarkPair"{ Some(Box::new(RandomMarkPair)) }
    else if s == "RandomCenterMarkPair" { Some(Box::new(RandomCenterMarkPair))}
    else if s == "UpperCenterMarkPair" { Some(Box::new(UpperCenterMarkPair)) }
    else if s == "LowerCenterMarkPair" {Some(Box::new(LowerCenterMarkPair)) }
    else{
        let divd : Vec<_> = s.split(" ").collect();
        if divd[0] == "BlackList" {
            Some(Box::new(BlackList(u8::from_str_radix(divd[1], 10).unwrap())))
        }
        else
        if divd[0] == "Set" {
            Some(Box::new(Set(
                        usize::from_str_radix(divd[1], 10).unwrap(),
                        usize::from_str_radix(divd[2], 10).unwrap(),
                        u8::from_str_radix(divd[3], 10).unwrap()
                        )))
        }
        else
        if divd[0] == "MarkPair" {
            Some(Box::new(MarkPair(
                        usize::from_str_radix(divd[1], 10).unwrap(),
                        usize::from_str_radix(divd[2], 10).unwrap(),
                        usize::from_str_radix(divd[3], 10).unwrap(),
                        usize::from_str_radix(divd[4], 10).unwrap()
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


fn c_instructions() -> LoteriaResult<Vec<Box<dyn ActDebug>>>{
    let mut file = std::fs::File::open(get_config_path()?).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let text = String::from_utf8(buffer).unwrap();
    let lines : Vec<_> = text.lines().collect();

    Ok(get_instructions(&lines))
}

pub fn run() -> LoteriaResult<Vec<Board>> {
    let mut args : Vec<_> = args().collect();
    args.remove(0);
    let instructions = if args.len() == 0{
        c_instructions()?
    }
    else{
        get_instructions(&args)
    };

    let mut board = BoardBuilder::new();
    for instruction in instructions{
        let _ = instruction.act_on(&mut board);
    }

    Ok(board.generate_boards())
}

mod test{

    #[test]
fn instruction_test(){
    use std::any::{TypeId, Any};
    use super::{get_instructions, ActDebug};
    use loteria_engine::engine::*;

    let lines = vec![
        "BlackList 8",
        "Set 2 3 7",
        "MarkPair 2 3 0 1",
        "RandomMarkPair",
        "RandomCenterMarkPair",
        "UpperCenterMarkPair",
        "LowerCenterMarkPair",
    ];
    let expected : Vec<Box<dyn ActDebug>> = vec![
        Box::new(BlackList(8)),
        Box::new(Set(2, 3, 7)),
        Box::new(MarkPair(2, 3, 0, 1)),
        Box::new(RandomMarkPair),
        Box::new(RandomCenterMarkPair),
        Box::new(UpperCenterMarkPair),
        Box::new(LowerCenterMarkPair),
    ];

    let instructions = get_instructions(lines.clone());
    for ((instr, ex), line) in instructions.iter().zip(&expected).zip(&lines){
        let i_any = instr as &dyn Any;
        let ex_any = ex as &dyn Any;
        assert!(i_any.type_id() == ex_any.type_id(), "error at: {line}");
    }
}
}
