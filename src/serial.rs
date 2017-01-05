use core::fmt;
use pins;
use spin;
use svd;

const BAUDRATE_9600_BAUD: u32 = 0x00275000;
const UART_ENABLE: u32 = 0x00000004;

lazy_static! {
    pub static ref SERIAL: spin::Mutex<Serial> = spin::Mutex::new(Serial::new());
}

pub struct Serial();

impl Serial {
    fn new() -> Serial {
        unsafe {
            pins::RX.input_pullup();
            pins::TX.output_pullup();
            svd::uart0().baudrate.write_bits(BAUDRATE_9600_BAUD);
            svd::uart0().enable.write_bits(UART_ENABLE);
            svd::uart0().tasks_starttx.write(1);
            svd::uart0().tasks_startrx.write(1);
            // Dummy write needed or TXDRDY trails write rather than leads write.
            // Pins are disconnected so nothing is physically transmitted on the wire.
            svd::uart0().txd.write_bits(0);
            svd::uart0().pselrxd.write(u32::from(pins::RX.0));
            svd::uart0().pseltxd.write(u32::from(pins::TX.0));
        }

        Serial()
    }

    pub fn writable(&self) -> bool {
        unsafe { svd::uart0().events_txdrdy.read() == 1 }
    }

    pub fn readable(&self) -> bool {
        unsafe { svd::uart0().events_rxdrdy.read() == 1 }
    }

    pub fn write_byte(&mut self, byte: u8) {
        while !self.writable() {}
        unsafe {
            svd::uart0().events_txdrdy.write(0);
            svd::uart0().txd.write_bits(u32::from(byte));
        }
    }

    #[allow(dead_code)]
    pub fn read_byte(&mut self) -> u8 {
        while !self.readable() {}
        unsafe {
            svd::uart0().events_rxdrdy.write(0);
            svd::uart0().rxd.read_bits() as u8
        }
    }
}

impl fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            self.write_byte(b)
        }
        Ok(())
    }
}
