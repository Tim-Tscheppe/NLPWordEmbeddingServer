/* Listen for HTTP Request on Port 8477 */
use std::net::TcpListener;

fn main() {
        /* Creating a Local TcpListener at Port 8477 */
        const HOST : &str ="127.0.0.1";
        const PORT : &str ="8477";
        /* Concating Host address and Port to Create Final Endpoint */
        let end_point : String = HOST.to_owned() + ":" +  PORT;
        /*Creating TCP Listener at our end point */
        let listener = TcpListener::bind(end_point).unwrap();
        println!("Web server is listening at port {}",PORT);
        /* Connecting to any incoming connections */
        for stream in listener.incoming() {
            let _stream = stream.unwrap();
            println!("Connection established!");
        }
}
