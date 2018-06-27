# guid-create

Rust helper for randomly creating GUIDs.

``` rust
extern crate guid_create;
use guid_create::GUID;

// Create GUIDs
let guid = GUID::rand();
let guid = GUID::parse("87935CDE-7094-4C2B-A0F4-DD7D512DD261").unwrap();
let guid = GUID::build_from_components(0x87935CDE, 0x7094, 0x4C2B, &[0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D, 0xD2, 0x61], );
let guid = GUID::build_from_slice(&[ 0x87, 0x93, 0x5C, 0xDE, 0x70, 0x94, 0x4C, 0x2B, 0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D, 0xD2, 0x61,]);

// View GUIDs
guid.to_string();  // 87935CDE-7094-4C2B-A0F4-DD7D512DD261

// Check GUIDs
guid.data1();
guid.data2();
guid.data3();
guid.data4();
```