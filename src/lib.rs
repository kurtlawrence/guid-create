//! # guid-create
//!
//! Rust helper for randomly creating GUIDs.
//!
//! ``` rust
//! extern crate guid_create;
//! use guid_create::GUID;
//!
//! // Create GUIDs
//! let guid = GUID::rand();
//! let guid = GUID::parse("87935CDE-7094-4C2B-A0F4-DD7D512DD261").unwrap();
//! let guid = GUID::build_from_components(0x87935CDE, 0x7094, 0x4C2B, &[0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D, 0xD2, 0x61], );
//! let guid = GUID::build_from_slice(&[ 0x87, 0x93, 0x5C, 0xDE, 0x70, 0x94, 0x4C, 0x2B, 0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D, 0xD2, 0x61,]);
//!
//! // View GUIDs
//! guid.to_string();  // 87935CDE-7094-4C2B-A0F4-DD7D512DD261
//!
//! // Check GUIDs
//! guid.data1();
//! guid.data2();
//! guid.data3();
//! guid.data4();
//! ```
#![deny(clippy::all, clippy::cargo)]
#![forbid(unsafe_code)]

extern crate byteorder;
extern crate chomp;
extern crate guid_parser;
extern crate rand;
#[cfg(windows)]
extern crate winapi;

use byteorder::{BigEndian, ByteOrder};
use chomp::prelude::*;
use std::{error, fmt};

#[cfg(windows)]
use winapi::shared::guiddef::GUID as WinGuid;

/// Parsing error type.
#[derive(Debug)]
pub struct ParseError;

impl error::Error for ParseError {
    fn description(&self) -> &str {
        "Malformed GUID, expecting XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX"
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Malformed GUID, expecting XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX"
        )
    }
}

/// A GUID backed by 16 byte array.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GUID {
    data: [u8; 16],
}

impl GUID {
    /// Construct a `GUID` from components.
    ///
    /// ```rust
    /// extern crate guid_create;
    /// let guid = guid_create::GUID::build_from_components(
    ///     0x87935CDE,
    ///     0x7094,
    ///     0x4C2B,
    ///     &[ 0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D, 0xD2, 0x61 ]
    /// );
    ///
    /// assert_eq!(guid.data1(), 0x87935CDE);
    /// assert_eq!(guid.data2(), 0x7094);
    /// assert_eq!(guid.data3(), 0x4C2B);
    /// assert_eq!(guid.data4(), [ 0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D, 0xD2, 0x61 ]);
    /// assert_eq!(guid.to_string(), "87935CDE-7094-4C2B-A0F4-DD7D512DD261");
    /// ```
    pub fn build_from_components(d1: u32, d2: u16, d3: u16, d4: &[u8; 8]) -> Self {
        let mut d = [0u8; 16];
        {
            // first data
            let mut buf = [0; 4];
            BigEndian::write_u32(&mut buf, d1);
            d[..4].clone_from_slice(&buf[..4]);
        }
        {
            // second data
            let mut buf = [0; 2];
            BigEndian::write_u16(&mut buf, d2);
            d[4..(2 + 4)].clone_from_slice(&buf[..2]);
        }
        {
            // third data
            let mut buf = [0; 2];
            BigEndian::write_u16(&mut buf, d3);
            d[6..(2 + 6)].clone_from_slice(&buf[..2]);
        }
        // fourth data
        d[8..(8 + 8)].clone_from_slice(&d4[..8]);

        GUID { data: d }
    }

    /// Construct a `GUID` from 16 bytes.
    ///
    /// ``` rust
    /// extern crate guid_create;
    /// let guid = guid_create::GUID::build_from_slice(&[
    ///     0x87, 0x93, 0x5C, 0xDE, 0x70, 0x94, 0x4C, 0x2B, 0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D,
    ///     0xD2, 0x61,
    /// ]);

    /// assert_eq!(guid.data1(), 0x87935CDE);
    /// assert_eq!(guid.data2(), 0x7094);
    /// assert_eq!(guid.data3(), 0x4C2B);
    /// assert_eq!(
    ///     guid.data4(),
    ///     [0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D, 0xD2, 0x61]
    /// );
    /// assert_eq!(guid.to_string(), "87935CDE-7094-4C2B-A0F4-DD7D512DD261");
    /// ```
    pub fn build_from_slice(data: &[u8; 16]) -> Self {
        let mut d = [0u8; 16];
        d[..16].clone_from_slice(&data[..16]);
        GUID { data: d }
    }

    /// Construct a `GUID` from a string.
    /// Leverages [`guid-parser`](https://docs.rs/guid-parser/0.1.0/guid_parser/index.html) for the parsing.
    ///
    /// ``` rust
    /// extern crate guid_create;
    /// let guid = guid_create::GUID::parse("87935CDE-7094-4C2B-A0F4-DD7D512DD261").unwrap();
    ///
    /// assert_eq!(guid.data1(), 0x87935CDE);
    /// assert_eq!(guid.data2(), 0x7094);
    /// assert_eq!(guid.data3(), 0x4C2B);
    /// assert_eq!(guid.data4(), [ 0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D, 0xD2, 0x61 ]);
    /// assert_eq!(guid.to_string(), "87935CDE-7094-4C2B-A0F4-DD7D512DD261");
    /// ```
    pub fn parse(guid: &str) -> Result<Self, ParseError> {
        let r = parse_only(guid_parser::chunks, guid.as_bytes());
        match r {
            Ok(chunks) => {
                let mut d4 = [0u8; 8];
                {
                    // chunk 4
                    let mut buf = [0; 2];
                    BigEndian::write_u16(&mut buf, chunks.chunk4);
                    d4[..2].clone_from_slice(&buf[..2]);
                }
                {
                    // hi data
                    let mut buf = [0; 2];
                    BigEndian::write_u16(&mut buf, chunks.chunk5.hi);
                    d4[2..(2 + 2)].clone_from_slice(&buf[..2]);
                }
                {
                    // lo data
                    let mut buf = [0; 4];
                    BigEndian::write_u32(&mut buf, chunks.chunk5.lo);
                    d4[4..(4 + 4)].clone_from_slice(&buf[..4]);
                }
                let guid =
                    GUID::build_from_components(chunks.chunk1, chunks.chunk2, chunks.chunk3, &d4);
                Ok(guid)
            }
            Err(_) => Err(ParseError),
        }
    }

    /// Generates a new GUID with 16 random bytes.
    pub fn rand() -> GUID {
        let mut d = [0u8; 16];
        for i in &mut d {
            *i = rand::random::<u8>();
        }

        GUID { data: d }
    }

    /// The first four bytes.
    ///
    /// ``` rust
    /// extern crate guid_create;
    /// let guid = guid_create::GUID::build_from_components(
    ///     500,
    ///     600,
    ///     700,
    ///     &[ 0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D, 0xD2, 0x61 ]
    /// );
    ///
    /// assert_eq!(guid.data1(), 500);
    /// ```
    pub fn data1(&self) -> u32 {
        BigEndian::read_u32(&self.data[0..4])
    }

    /// Bytes 5 and 6.
    ///
    /// ``` rust
    /// extern crate guid_create;
    /// let guid = guid_create::GUID::build_from_components(
    ///     500,
    ///     600,
    ///     700,
    ///     &[ 0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D, 0xD2, 0x61 ]
    /// );
    ///
    /// assert_eq!(guid.data2(), 600);
    /// ```
    pub fn data2(&self) -> u16 {
        BigEndian::read_u16(&self.data[4..6])
    }

    /// Bytes 7 and 8.
    ///
    /// ``` rust
    /// extern crate guid_create;
    /// let guid = guid_create::GUID::build_from_components(
    ///     500,
    ///     600,
    ///     700,
    ///     &[ 0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D, 0xD2, 0x61 ]
    /// );
    ///
    /// assert_eq!(guid.data3(), 700);
    /// ```
    pub fn data3(&self) -> u16 {
        BigEndian::read_u16(&self.data[6..8])
    }

    /// The last eight bytes.
    ///
    /// ``` rust
    /// extern crate guid_create;
    /// let guid = guid_create::GUID::build_from_components(
    ///     500,
    ///     600,
    ///     700,
    ///     &[ 0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D, 0xD2, 0x61 ]
    /// );
    ///
    /// assert_eq!(
    ///     guid.data4(),
    ///     [0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D, 0xD2, 0x61]
    /// );
    /// ```
    pub fn data4(&self) -> [u8; 8] {
        let mut arr = [0u8; 8];
        arr[..8].clone_from_slice(&self.data[8..(8 + 8)]);
        arr
    }

    /// Convert the `GUID` to a `winapi` [GUID](https://docs.rs/winapi/0.3.4/x86_64-pc-windows-msvc/winapi/shared/guiddef/struct.GUID.html)
    /// > Only present on windows targets
    ///
    /// ```rust
    /// extern crate guid_create;
    /// let guid = guid_create::GUID::build_from_components(
    ///     0x87935CDE,
    ///     0x7094,
    ///     0x4C2B,
    ///     &[ 0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D, 0xD2, 0x61 ]
    /// );
    ///
    /// let win = guid.as_winapi_guid();
    /// assert_eq!(win.Data1, 0x87935CDE);
    /// assert_eq!(win.Data2, 0x7094);
    /// assert_eq!(win.Data3, 0x4C2B);
    /// assert_eq!(win.Data4, [ 0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D, 0xD2, 0x61 ]);
    /// ```
    #[cfg(windows)]
    pub fn as_winapi_guid(&self) -> WinGuid {
        WinGuid {
            Data1: self.data1(),
            Data2: self.data2(),
            Data3: self.data3(),
            Data4: self.data4(),
        }
    }

    /// Convert a `winapi` [GUID](https://docs.rs/winapi/0.3.4/x86_64-pc-windows-msvc/winapi/shared/guiddef/struct.GUID.html) to a `GUID`
    /// > Only present on windows targets
    ///
    /// ``` rust
    /// extern crate guid_create;
    /// extern crate winapi;
    /// let win = winapi::shared::guiddef::GUID {
    ///     Data1: 0x87935CDE,
    ///     Data2: 0x7094,
    ///     Data3: 0x4C2B,
    ///     Data4: [ 0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D, 0xD2, 0x61 ]
    /// };
    ///
    /// let guid = guid_create::GUID::from_winapi_guid(win);
    /// assert_eq!(guid.data1(), 0x87935CDE);
    /// assert_eq!(guid.data2(), 0x7094);
    /// assert_eq!(guid.data3(), 0x4C2B);
    /// assert_eq!(guid.data4(), [ 0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D, 0xD2, 0x61 ]);
    /// ```
    #[cfg(windows)]
    pub fn from_winapi_guid(guid: WinGuid) -> Self {
        GUID::build_from_components(guid.Data1, guid.Data2, guid.Data3, &guid.Data4)
    }
}

#[cfg(feature = "serde")]
extern crate serde;
#[cfg(feature = "serde")]
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for GUID {
    fn deserialize<D>(deserializer: D) -> Result<GUID, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string_guid = String::deserialize(deserializer)?;
        let guid = GUID::parse(&string_guid)
            .map_err(|_| de::Error::custom(format!("cannot convert {} to guid", string_guid)))?;
        Ok(guid)
    }
}

#[cfg(feature = "serde")]
impl Serialize for GUID {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&*self.to_string())
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

    #[cfg(windows)]
    #[test]
    fn win_guid() {
        for _ in 0..10000 {
            let guid = GUID::rand();
            let win = guid.as_winapi_guid();
            assert_eq!(guid.data1(), win.Data1);
            assert_eq!(guid.data2(), win.Data2);
            assert_eq!(guid.data3(), win.Data3);
            assert_eq!(guid.data4(), win.Data4);
            let convert_back = GUID::from_winapi_guid(win);
            assert_eq!(guid, convert_back);
        }
    }

    #[test]
    fn create_from_components() {
        let guid = GUID::build_from_components(
            0x87935CDE,
            0x7094,
            0x4C2B,
            &[0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D, 0xD2, 0x61],
        );

        assert_eq!(guid.data1(), 0x87935CDE);
        assert_eq!(guid.data2(), 0x7094);
        assert_eq!(guid.data3(), 0x4C2B);
        assert_eq!(
            guid.data4(),
            [0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D, 0xD2, 0x61]
        );
        assert_eq!(guid.to_string(), "87935CDE-7094-4C2B-A0F4-DD7D512DD261");
    }

    #[test]
    fn create_from_array() {
        let guid = GUID::build_from_slice(&[
            0x87, 0x93, 0x5C, 0xDE, 0x70, 0x94, 0x4C, 0x2B, 0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D,
            0xD2, 0x61,
        ]);

        println!("{}", guid);

        assert_eq!(guid.data1(), 0x87935CDE);
        assert_eq!(guid.data2(), 0x7094);
        assert_eq!(guid.data3(), 0x4C2B);
        assert_eq!(
            guid.data4(),
            [0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D, 0xD2, 0x61]
        );
        assert_eq!(guid.to_string(), "87935CDE-7094-4C2B-A0F4-DD7D512DD261");
    }

    #[test]
    fn parse_strings() {
        for _ in 0..10000 {
            let guid = GUID::rand();
            let s = guid.to_string();
            let guid2 = GUID::parse(&s).unwrap();
            assert_eq!(guid, guid2);
        }
    }
}
