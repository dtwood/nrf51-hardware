use svd::gpio;

unsafe fn write_pin_cnf(number: u8, bits: u32) {
    match number {
        0 => gpio().pin_cnf0.write_bits(bits),
        1 => gpio().pin_cnf1.write_bits(bits),
        2 => gpio().pin_cnf2.write_bits(bits),
        3 => gpio().pin_cnf3.write_bits(bits),
        4 => gpio().pin_cnf4.write_bits(bits),
        5 => gpio().pin_cnf5.write_bits(bits),
        6 => gpio().pin_cnf6.write_bits(bits),
        7 => gpio().pin_cnf7.write_bits(bits),
        8 => gpio().pin_cnf8.write_bits(bits),
        9 => gpio().pin_cnf9.write_bits(bits),
        10 => gpio().pin_cnf10.write_bits(bits),
        11 => gpio().pin_cnf11.write_bits(bits),
        12 => gpio().pin_cnf12.write_bits(bits),
        13 => gpio().pin_cnf13.write_bits(bits),
        14 => gpio().pin_cnf14.write_bits(bits),
        15 => gpio().pin_cnf15.write_bits(bits),
        16 => gpio().pin_cnf16.write_bits(bits),
        17 => gpio().pin_cnf17.write_bits(bits),
        18 => gpio().pin_cnf18.write_bits(bits),
        19 => gpio().pin_cnf19.write_bits(bits),
        20 => gpio().pin_cnf20.write_bits(bits),
        21 => gpio().pin_cnf21.write_bits(bits),
        22 => gpio().pin_cnf22.write_bits(bits),
        23 => gpio().pin_cnf23.write_bits(bits),
        24 => gpio().pin_cnf24.write_bits(bits),
        25 => gpio().pin_cnf25.write_bits(bits),
        26 => gpio().pin_cnf26.write_bits(bits),
        27 => gpio().pin_cnf27.write_bits(bits),
        28 => gpio().pin_cnf28.write_bits(bits),
        29 => gpio().pin_cnf29.write_bits(bits),
        30 => gpio().pin_cnf30.write_bits(bits),
        31 => gpio().pin_cnf31.write_bits(bits),
        _ => unreachable!(),
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PinNumber(pub u8);

impl PinNumber {
    fn mask(&self) -> u32 {
        1 << self.0
    }

    pub fn input_pullup(&self) {
        unsafe {
            write_pin_cnf(self.0, CONFIG_INPUT | CONFIG_PULLUP);
        }
    }

    pub fn output_pullup(&self) {
        unsafe {
            write_pin_cnf(self.0, CONFIG_OUTPUT | CONFIG_PULLUP);
        }
    }
}

pub struct Pin {
    mask: u32,
}

impl Pin {
    pub fn output(number: PinNumber) -> Self {
        unsafe {
            write_pin_cnf(number.0, CONFIG_OUTPUT);
        }
        Pin { mask: number.mask() }
    }

    pub fn set_high(&self) {
        unsafe {
            gpio().outset.write_bits(self.mask);
        }
    }

    pub fn set_low(&self) {
        unsafe {
            gpio().outclr.write_bits(self.mask);
        }
    }
}


const CONFIG_INPUT: u32 = 0 << 0;
const CONFIG_OUTPUT: u32 = 1 << 0;
const CONFIG_PULLUP: u32 = 3 << 2;
