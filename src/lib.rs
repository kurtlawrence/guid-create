extern crate byteorder;
extern crate rand;
extern crate winapi;

use byteorder::{ByteOrder, NativeEndian};
use std::fmt;
use winapi::shared::guiddef::GUID as WinGuid;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct GUID {
	data: [u8; 16],
}

impl GUID {
	pub fn build_from_components(d1: u32, d2: u16, d3: u16, d4: &[u8; 8]) -> Self {
		let mut d = [0u8; 16];
		{
			// first data
			let mut buf = [0; 4];
			NativeEndian::write_u32(&mut buf, d1);
			for i in 0..4 {
				d[i] = buf[i];
			}
		}
		{
			// second data
			let mut buf = [0; 2];
			NativeEndian::write_u16(&mut buf, d2);
			for i in 0..2 {
				d[i + 4] = buf[i];
			}
		}
		{
			// third data
			let mut buf = [0; 2];
			NativeEndian::write_u16(&mut buf, d3);
			for i in 0..2 {
				d[i + 6] = buf[i];
			}
		}
		// fourth data
		for i in 0..8 {
			d[i + 8] = d4[i];
		}

		GUID { data: d }
	}

	pub fn build_from_slice(data: &[u8; 16]) -> Self {
		let mut d = [0u8; 16];
		for i in 0..16 {
			d[i] = data[i];
		}
		GUID { data: d }
	}

	pub fn rand() -> GUID {
		let mut d = [0u8; 16];
		for i in 0..16 {
			d[i] = rand::random::<u8>();
		}

		GUID { data: d }
	}

	pub fn data1(&self) -> u32 {
		NativeEndian::read_u32(&self.data[0..4])
	}

	pub fn data2(&self) -> u16 {
		NativeEndian::read_u16(&self.data[4..6])
	}

	pub fn data3(&self) -> u16 {
		NativeEndian::read_u16(&self.data[6..8])
	}

	pub fn data4(&self) -> [u8; 8] {
		let mut arr = [0u8; 8];
		for i in 0..8 {
			arr[i] = self.data[i + 8];
		}
		arr
	}

	pub fn as_winapi_guid(&self) -> WinGuid {
		WinGuid {
			Data1: self.data1(),
			Data2: self.data2(),
			Data3: self.data3(),
			Data4: self.data4(),
		}
	}

	pub fn from_winapi_guid(guid: WinGuid) -> Self {
		GUID::build_from_components(guid.Data1, guid.Data2, guid.Data3, &guid.Data4)
	}
}

impl fmt::Display for GUID {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut s1 = String::new();
		for i in 0..2 {
			s1.push_str(&format!("{:02X}", self.data4()[i]))
		}
		let mut s2 = String::new();
		for i in 2..8 {
			s2.push_str(&format!("{:02X}", self.data4()[i]))
		}
		write!(
			f,
			"{:08X}-{:04X}-{:04X}-{}-{}",
			self.data1(),
			self.data2(),
			self.data3(),
			s1,
			s2
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn travis_test() {}

	#[test]
	fn string_lengths() {
		for _ in 0..10000 {
			let guid = GUID::rand();
			let s = guid.to_string();
			println!("{}", s);
			assert_eq!(s.len(), 36);
		}
	}

	#[test]
	fn win_guid() {
		for _ in 0..10000 {
			let guid = GUID::rand();
			let win = guid.as_winapi_guid();
			let convert_back = GUID::from_winapi_guid(win);
			assert_eq!(guid, convert_back);
		}
	}

}
