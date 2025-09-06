#[macro_export]
macro_rules! eterm{
    (clear) => {stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap()}; 
    (flush) => {stdout().flush().unwrap()};
    (bold)  => {stdout().execute(SetAttribute(Attribute::Bold))};
    (size)  => {terminal::size().unwrap()};
    (raw)   => {enable_raw_mode().unwrap()};
    (unraw) => {disable_raw_mode().unwrap()};
    (underline) => {stdout().execute(SetAttribute(Attribute::underline))};
    (move($x:expr,$y:expr))=> {stdout().execute(cursor::MoveTo($y,$x)).unwrap()};
    (color(fg,$color:ident)) =>{stdout().execute(SetForegroundColor(Color::$color)).unwrap()};
    (color(bg,$color:ident)) =>{stdout().execute(SetBackgroundColor(Color::$color)).unwrap()};
    (print($string:literal)) => {stdout().execute(Print($string.to_string())).unwrap()};
    (poll($time:expr)) => {poll(Duration::from_millis($time)).unwrap()};
    //(keyInput) => 
}
macro_rules! input{
    ($t:ty) => {{
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<$t>().unwrap()
    }};
}
macro_rules! string{
    ($s:literal)=> {($s).to_string();};
}
