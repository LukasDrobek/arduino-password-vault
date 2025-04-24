// use serialport::SerialPort;
use std::io::{self, BufRead, BufReader, Write};
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let ports = serialport::available_ports()?;
    let port_name = ports
        .into_iter()
        .find(|p| p.port_name.contains("ttyACM") || p.port_name.contains("ttyUSB"))
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Arduino not found"))?
        .port_name;

    println!("Using port {}", port_name);

    let mut port = serialport::new(port_name, 115200)
        .timeout(Duration::from_millis(1000))
        .open()?;

    let mut reader = BufReader::new(port.try_clone()?);
    loop {
        print!("Enter an integer (or 'q' to quit): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "q" {
            println!("Shutting down...");
            break;
        }

        port.write_all(input.as_bytes())?;
        port.write_all(b"\n")?;
        port.flush()?;

        let mut response = String::new();
        reader.read_line(&mut response)?;
        let num: i32 = match response.trim().parse() {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error parsing response {}", e);
                continue;
            }
        };

        println!("Arduino reponse: {}", num);
    }

    Ok(())       
}
