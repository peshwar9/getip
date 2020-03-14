/*
This progam does two things:
1) Gets IP address of current machine by invoking an API call to external service
2) Gets host name of current machine by spawning a child process and executing the hostname command.
*/

use requests::*;
use std::process::Command;

fn main() {
    // Make call to 3rd party service to get ip address
    let response = requests::get("https://api.ipify.org?format=json").unwrap();
    //Extract json response
    let data = response.json().unwrap();
    //Print value after converting to string
    println!("Ip address is {:?}", data["ip"].to_string());

    //To get hostname

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "hostname"])
            .output()
            .expect("Unable to get hostname, sorry")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("hostname")
            .output()
            .expect("Unable to get hostname, sorry")
    };

    println!("Hostname is {:?}", String::from_utf8(output.stdout));
}
