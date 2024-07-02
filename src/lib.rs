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
#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::{convert::TryInto, fmt};

#[cfg(windows)]
use winapi::shared::guiddef::GUID as WinGuid;

/// Parsing error type.
#[derive(Debug)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Malformed GUID, expecting XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX"
        )
    }
}

impl std::error::Error for ParseError {}

/// A GUID backed by 16 byte array.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct GUID {
    data: [u8; 16],
}

impl GUID {
    /// Construct a `GUID` from components.
    ///
    /// ``` rust
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
    pub const fn build_from_components(d1: u32, d2: u16, d3: u16, d4: &[u8; 8]) -> Self {
        let d1 = d1.to_be_bytes();
        let d2 = d2.to_be_bytes();
        let d3 = d3.to_be_bytes();
        let data = [
            d1[0], d1[1], d1[2], d1[3],
            d2[0], d2[1],
            d3[0], d3[1],
            d4[0], d4[1], d4[2], d4[3], d4[4], d4[5], d4[6], d4[7],
        ];

        GUID{ data }
    }

    /// Construct a `GUID` from 16 bytes.
    ///
    /// ``` rust
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
    pub const fn build_from_slice(data: &[u8; 16]) -> Self {
        GUID { data: *data }
    }

    /// Construct a `GUID` from a string.
    ///
    /// ``` rust
    /// let guid = guid_create::GUID::parse("87935CDE-7094-4C2B-A0F4-DD7D512DD261").unwrap();
    ///
    /// assert_eq!(guid.data1(), 0x87935CDE);
    /// assert_eq!(guid.data2(), 0x7094);
    /// assert_eq!(guid.data3(), 0x4C2B);
    /// assert_eq!(guid.data4(), [ 0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D, 0xD2, 0x61 ]);
    /// assert_eq!(guid.to_string(), "87935CDE-7094-4C2B-A0F4-DD7D512DD261");
    /// ```
    pub fn parse(guid: &str) -> Result<Self, ParseError> {
        fn n(ch: u8) -> Result<u8, ParseError> {
            match ch {
                b'0'..=b'9' => Ok(ch - 48),
                b'A'..=b'F' => Ok(ch - 55),
                b'a'..=b'f' => Ok(ch - 87),
                _ => Err(ParseError),
            }
        }
        fn hexbyte(s: &[u8]) -> Result<(u8, &[u8]), ParseError> {
            match s {
                [a, b, tail @ ..] => n(*a)
                    .and_then(|a| n(*b).map(|b| a * 16 + b))
                    .map(|x| (x, tail)),
                _ => Err(ParseError),
            }
        }
        fn strip_dash(s: &[u8]) -> Result<&[u8], ParseError> {
            match s {
                [b'-', tail @ ..] => Ok(tail),
                _ => Err(ParseError),
            }
        }

        let mut data = [0u8; 16];

        let mut s = guid.as_bytes();

        fn fill<'a>(buf: &mut [u8], mut s: &'a [u8]) -> Result<&'a [u8], ParseError> {
            for l in buf {
                let (d, s_) = hexbyte(s)?;
                *l = d;
                s = s_;
            }
            Ok(s)
        }

        // first four bytes
        s = fill(&mut data[..4], s)?;
        s = strip_dash(s)?;

        // second two bytes
        s = fill(&mut data[4..6], s)?;
        s = strip_dash(s)?;

        // third two bytes
        s = fill(&mut data[6..8], s)?;
        s = strip_dash(s)?;

        // fourth two bytes
        s = fill(&mut data[8..10], s)?;
        s = strip_dash(s)?;

        // trailing bytes
        s = fill(&mut data[10..], s)?;

        // should be empty!
        if s.is_empty() {
            Ok(Self { data })
        } else {
            Err(ParseError)
        }
    }

    /// Generates a new GUID with 16 random bytes.
    pub fn rand() -> GUID {
        GUID {
            data: rand::random(),
        }
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
        u32::from_be_bytes(
            self.data[0..4]
                .try_into()
                .expect("slice with incorrect length"),
        )
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
        u16::from_be_bytes(
            self.data[4..6]
                .try_into()
                .expect("slice with incorrect length"),
        )
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
        u16::from_be_bytes(
            self.data[6..8]
                .try_into()
                .expect("slice with incorrect length"),
        )
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
        self.data[8..16]
            .try_into()
            .expect("slice with incorrect length")
    }

    /// Convert the `GUID` to a `winapi` [GUID](https://docs.rs/winapi/0.3.4/x86_64-pc-windows-msvc/winapi/shared/guiddef/struct.GUID.html)
    /// > Only present on windows targets
    ///
    /// ``` rust
    /// extern crate guid_create;
    /// let guid = guid_create::GUID::build_from_components(
    /// 	0x87935CDE,
    /// 	0x7094,
    /// 	0x4C2B,
    /// 	&[ 0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D, 0xD2, 0x61 ]
    /// 	);
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
    /// 	Data1: 0x87935CDE,
    /// 	Data2: 0x7094,
    /// 	Data3: 0x4C2B,
    /// 	Data4: [ 0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D, 0xD2, 0x61 ]
    /// 	};
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
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for GUID {
    fn deserialize<D>(deserializer: D) -> Result<GUID, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string_guid = String::deserialize(deserializer)?;
        let guid = GUID::parse(&string_guid)
            .map_err(|_| de::Error::custom(format!("cannot convert {string_guid} to guid")))?;
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
        write!(
            f,
            "{:08X}-{:04X}-{:04X}-{:04X}-{:08X}{:04X}",
            self.data1(),
            self.data2(),
            self.data3(),
            u16::from_be_bytes(self.data[8..10].try_into().unwrap()),
            u32::from_be_bytes(self.data[10..14].try_into().unwrap()),
            u16::from_be_bytes(self.data[14..16].try_into().unwrap()),
        )
    }
}

#[cfg(test)]
impl quickcheck::Arbitrary for GUID {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let mut data = [0u8; 16];
        data.fill_with(|| u8::arbitrary(g));
        Self { data }
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

    #[quickcheck]
    fn no_panic_parse(s: String) {
        GUID::parse(&s).ok();
    }

    #[quickcheck]
    fn parse_success(guid: GUID) -> bool {
        let s = guid.to_string();
        let g2 = GUID::parse(&s).unwrap();
        g2 == guid
    }
}
