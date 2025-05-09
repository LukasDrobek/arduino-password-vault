use std::time::Duration;
use serialport::SerialPort;
use anyhow::Result;
use crate::constants::{BAUD_RATE, SALT_LEN};

pub struct SerialManager {
    port: Box<dyn SerialPort>,
}

impl SerialManager {
    pub fn new() -> Result<Self> {
        let ports = serialport::available_ports()?;
        let port_info = ports
            .into_iter()
            .find(|p| p.port_name.contains("ttyACM") || p.port_name.contains("ttyUSB"))
            .ok_or_else(|| anyhow::anyhow!("No AMC/USB serial port found."))?;
        
        let port = serialport::new(port_info.port_name, BAUD_RATE)
            .timeout(Duration::from_millis(500))
            .open()?;

        Ok(Self { port })
    }

    pub fn write(&mut self, data: &str) -> Result<()> {
        self.port.write_all(data.as_bytes())?;
        self.port.flush()?;
        Ok(())
    }

    pub fn read_line(&mut self) -> Result<String> {
        let mut buf = String::new();
        self.port.read_to_string(&mut buf)?;
        Ok(buf.trim().to_string())
    }

    pub fn get_salt(&mut self) -> Result<[u8; SALT_LEN]> {
        self.write("GET_SALT\n")?;
        let mut salt = [0u8; SALT_LEN];
        self.port.read_exact(&mut salt);
        Ok(salt)
    }
}
