use std::env;
use std::net::{IpAddr, TcpStream};
use std::str::FromStr; //trait to convert string to corresponding type
use std::process;
use std::sync::mpsc; //multiple producer, single consumer
use std::thread;
use std::io::{self, Write};
const MAX: u16 = 65535;

struct Args {
    flag: String,
    ipaddr: IpAddr, //enum IPv4, IPv6
    threads: u16,
}

impl Args {
    fn new(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }

        let flag = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&flag) {
            return Ok(Args {
                flag: String::from(""),
                ipaddr,
                threads: 4,
            });
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("--help") && args.len() == 2 {
                println!(
                    "Usage: -j to select how many threads you want
                \n -h or --help to show this help message"
                );
                return Err("help");
            } else if flag.contains("-h") || flag.contains("--help") {
                return Err("too many arguments");
            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IPADDR; must be IPv4 or IPv6"),
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("failed to parse thread number"),
                };
                return Ok(Args {
                    threads,
                    flag,
                    ipaddr,
                });
            } else {
                return Err("invalid syntax");
            }
        }
    }
}

fn scan(tx: mpsc::Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port +1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {} //do nothing
        }

        if (MAX - port) <= num_threads {
            break;
        }

        port += num_threads; 
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone(); //path to the executable

    let arguments = Args::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0);
            } else {
                eprintln!("{} problem parsing arguments: {}", program, err);
                process::exit(0);
            }
        }
    );

    let num_threads = arguments.threads;
    let (tx, rx) = mpsc::channel(); //tx is transmitter, rx is receiver

    for i in 0..num_threads{
        let tx = tx.clone(); //cloning the transmitter
        thread::spawn(move || {
            scan(tx, i, arguments.ipaddr, num_threads);
        });
    }
    
    let mut out =vec![];
    drop(tx); //drop the transmitter so that the receiver knows that there are no more messages

    for p in rx {
        out.push(p); //push the port number to the vector
    }

    println!("");
    out.sort();
    for v in out{
        println!("{} is open", v);
    }
}
