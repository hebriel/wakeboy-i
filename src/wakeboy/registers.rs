use super::core::*;

#[derive(Copy, Clone, std::cmp::PartialEq)]
pub enum Reg8 {
	B, C, D,
	E, H, L,
	MHL, A, Invalid
}

#[derive(Copy, Clone, std::cmp::PartialEq)]
pub enum Reg16 {
	BC, DE, HL,
	SP, AF, Invalid
}

#[derive(Default)]
pub struct Registers {
	pub a: u8,
	pub b: u8,
	pub c: u8,
	pub d: u8,
	pub e: u8,
	pub f: u8,
	pub sp: u16,
	pub pc: u16,
}

impl std::convert::From<u8> for Reg8 {
	fn from(bits: u8) -> Self {
		match bits {
			0b000 => Reg8::B,
			0b001 => Reg8::C,
			0b010 => Reg8::D,
			0b011 => Reg8::E,
			0b100 => Reg8::H,
			0b101 => Reg8::L,
			0b110 => Reg8::MHL,
			0b111 => Reg8::A,
			_ => {
				warn_or_crash(String::from("Program encountered invalid register denominator"));
				Reg8::Invalid
			}
		}
	}
}

impl Reg16 {
	pub fn from(bits: u8, want_sp: bool) -> Self {
		match bits {
			0b00 => Reg16::BC,
			0b01 => Reg16::DE,
			0b10 => Reg16::HL,
			0b11 => {
				if want_sp { return Reg16::SP } else { return Reg16::AF }
			}
			_ => {
				warn_or_crash(String::from("Program encountered invalid register denominator"));
				Reg16::Invalid
			}
		}
	}
}

impl std::fmt::Display for Reg8 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ret = match *self {
			Reg8::A => "A",
			Reg8::B => "B",
			Reg8::C => "C",
			Reg8::D => "D",
			Reg8::E => "E",
			Reg8::H => "H",
			Reg8::L => "L",
			Reg8::MHL => "(HL)",
			Reg8::Invalid => "Invalid Reg8",
		};
		write!(f, "{}", ret)
    }
}

impl std::fmt::Display for Reg16 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ret = match *self {
			Reg16::AF => "AF",
			Reg16::BC => "BC",
			Reg16::DE => "DE",
			Reg16::HL => "HL",
			Reg16::SP => "SP",
			Reg16::Invalid => "Invalid Reg16",
		};
		write!(f, "{}", ret)
    }
}