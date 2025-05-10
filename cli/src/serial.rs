use crate::constants::BAUD_RATE;
use anyhow::{anyhow, Result};
use serialport::SerialPort;
use std::io::ErrorKind::TimedOut;
use std::time::Duration;

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

    pub fn write_str(&mut self, data: &str) -> Result<()> {
        self.port.write_all(data.as_bytes())?;
        self.port.flush()?;
        Ok(())
    }

    pub fn write_bytes(&mut self, data: &[u8]) -> Result<()> {
        self.port.write_all(&data)?;
        self.port.flush()?;
        Ok(())
    }

    pub fn read_line(&mut self) -> Result<String> {
        let mut buf = Vec::new();
        let mut byte = [0u8];

        loop {
            match self.port.read(&mut byte) {
                Ok(0) => continue,
                Ok(1) => {
                    if byte[0] == b'\n' {
                        break;
                    } else {
                        buf.push(byte[0]);
                    }
                }
                Err(ref e) if e.kind() == TimedOut => continue,
                Err(e) => return Err(e.into()),
                _ => unreachable!(),
            }
        }

        let line = String::from_utf8(buf)?;
        Ok(line)
    }

    pub fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        let mut bytes_read = 0;
        let buf_len = buf.len();
        
        while bytes_read < buf_len {
            match self.port.read(&mut buf[bytes_read..]) {
                Ok(0) => continue,
                Ok(n) => bytes_read += n,
                Err(ref e) if e.kind() == TimedOut => continue,
                Err(e) => return Err(anyhow!("Serial read exact failed: {}", e)),
            }
        }
        
        Ok(())
    }
}
