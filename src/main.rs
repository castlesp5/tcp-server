use std::io;
use std::net::TcpListener;
use std::{
    io::BufRead,
    env
};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {panic!("error on argument level");}

    let listener = TcpListener::bind(format!("{}", &args[1])).expect("ERROR ON FINDING THE SERVER");
    println!("Server is listening on {}...", &args[1]);

    for stream in listener.incoming(){
        let stream = stream?;
        println!("connection established with {:?}", stream);
        let streambuff = io::BufReader::new(&stream);
        let http_request: Vec<_> = streambuff
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        println!("Recieved : {:#?}", http_request);
    }
    Ok(())

}
