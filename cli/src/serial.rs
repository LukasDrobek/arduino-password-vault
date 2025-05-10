use std::time::Duration;
use serialport::SerialPort;
use anyhow::Result;
use crate::constants::BAUD_RATE;

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
        let mut buf = String::new();
        self.port.read_to_string(&mut buf)?;
        Ok(buf.trim().to_string())
    }

    pub fn read_exact(&mut self, buf: &mut [u8]) -> anyhow::Result<()> {
        self.port.read_exact(buf)
            .map_err(|e| anyhow::anyhow!("Serial read_exact failed: {}", e))?;
        Ok(())
    }
}
