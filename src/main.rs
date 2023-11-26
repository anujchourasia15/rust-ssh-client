use std::io::Read;
use std::net::TcpStream;
use ssh2::Session;

// Authenticate the SSH session using a username and password.
fn authenticate(session: &mut Session) {
    match session.userauth_password("username", "password") {    // replace with actual credentials required to authenticate to the SSH server you are trying to connect to
        Ok(_authenticate) => println!("Authenticated"),
        Err(e) => {
            eprint!("Failed to authenticate: {:?}", e);
            std::process::exit(1);
        }
    }
}

// Establish a TCP connection to the SSH server based on the provided IPv4 address
fn tcpstream(ipv4: &str) -> TcpStream {
    match TcpStream::connect(format!("{}:22", ipv4)) {
        Ok(tcpstream) => tcpstream,
        Err(e) => {
            eprint!("Failed to connect: {}", e);
            std::process::exit(1);
        }
    }
}

// Create an SSH session
fn create_session() -> Session {
    match Session::new() {
        Ok(session) => session,
        Err(e) => {
            eprint!("Failed to create a session: {}", e);
            std::process::exit(1);
        }
    }
}

fn main() {
    let ipv4 = String::from("192.168.1.1");         
    let tcp_stream = tcpstream(&ipv4);

    println!("Connected to Server");

    let mut session = create_session();

    // set up session with tcp stream and does handshake
    session.set_tcp_stream(tcp_stream);
    session.handshake().unwrap();

    authenticate(&mut session);

    // Establish SSH channel and execute 'ls' command then read the output and print it.
    let _channel = match session.channel_session(){
        Ok(mut channel) => {
            channel.exec("ls").unwrap();
            let mut s = String::new();
            channel.read_to_string(&mut s).unwrap();
            println!("{}", s);
            channel.wait_close().unwrap();

            let exit_status = channel.exit_status().unwrap();
            if exit_status != 0 {
                eprint!("Exited with status {}", exit_status);
            }   
        }
        Err(e) => {
            eprint!("Failed to create channel: {}", e);
            return;
        }
    };
}
