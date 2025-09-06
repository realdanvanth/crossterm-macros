#![allow(unused_imports)]
use crossterm::{ExecutableCommand,terminal,cursor};
use crossterm::style::{Print,SetBackgroundColor,SetForegroundColor,Color};
use std::io::{stdout,Write};
#[macro_export]
macro_rules! eterm{
    (clear) => {stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap()}; 
    (flush) => {stdout().flush().unwrap()};
    (bold)  => {stdout().execute(SetAttribute(Attribute::Bold))};
    (underline) => {stdout().execute(SetAttribute(Attribute::underline))};
    (move($s:expr,$s1:expr))=> {stdout().execute(cursor::MoveTo($s,$s1)).unwrap()};
    (color(fg,$s:ident)) =>{stdout().execute(SetForegroundColor(Color::$s)).unwrap()};
    (color(bg,$s:ident)) =>{stdout().execute(SetBackgroundColor(Color::$s)).unwrap()};
    (print($s:literal)) => {stdout().execute(Print($s.to_string())).unwrap()};
}
fn main(){
    eterm!(clear);
    eterm!(color(fg,Red));
    eterm!(move(0,0));
    println!("hello");
    eterm!(flush);
}
