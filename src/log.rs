
use std::{rc::Rc, sync::Mutex};

pub enum Level{
    LOG,
    WARNING,
    ERROR,
    PANIC,
}

pub struct LogBuffer{
    str: String,
    out: Box<dyn std::io::Write>,
}

pub struct Logger{
    out: Rc<Mutex<LogBuffer>>,
}


impl Logger{
    pub fn nl(&mut self, l: Level, msg: String){
    }
}




