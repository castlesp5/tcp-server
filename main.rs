use std::{env, io, net::TcpListener};
use std::io::BufRead;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {panic!("error on argument level");}

    let listener = TcpListener::bind(&args[1]).unwrap();
    println!("Server is listening on {}...", &args[1]);

    for stream in listener.incoming(){
        let stream = stream?;
        println!("connection established with {:?}", stream.peer_addr()?);
        let streambuffer = io::BufReader::new(&stream);
        let request: Vec<_> = streambuffer.lines().collect();
        for message in request {
            println!("Recieved from {:?} ---> {:?}", stream.peer_addr()?, message?);
        }
    }
    Ok(())
}
