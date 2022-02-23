mod models;

//use sqlite;
use models::{span::Span, time::Time};

fn main() {
    let span = Span::new(Time::new(3, 4), "3:05".parse().unwrap());
    
    println!("{}", span);
    //let connection = sqlite::Connection::open_with_flags(
        //"../data/database.db",
        //sqlite::OpenFlags::new().set_read_only(),
    //)
    //.unwrap();

    //let mut cursor = connection
        //.prepare("SELECT * FROM buildings")
        //.unwrap()
        //.into_cursor();
    //let row = cursor.next().unwrap().unwrap();
    //println!("{}", row[0].as_string().unwrap());
}
