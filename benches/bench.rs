#![feature(test)]

extern crate guid_create;
extern crate test;

#[cfg(test)]
mod tests {
    use guid_create::GUID;
    use test::Bencher;

    #[bench]
    fn bench_rand(b: &mut Bencher) {
        b.iter(|| {
            GUID::rand();
        });
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        let guid = GUID::rand();
        let s = guid.to_string();
        b.iter(|| {
            GUID::parse(&s).unwrap();
        })
    }

    #[bench]
    fn bench_build_from_array(b: &mut Bencher) {
        let slice = [
            0x87, 0x93, 0x5C, 0xDE, 0x70, 0x94, 0x4C, 0x2B, 0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D,
            0xD2, 0x61,
        ];

        b.iter(|| {
            GUID::build_from_slice(&slice);
        })
    }

    #[bench]
    fn bench_build_from_components(b: &mut Bencher) {
        let d1 = 0x87935CDE;
        let d2 = 0x7094;
        let d3 = 0x4C2B;
        let d4 = [0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D, 0xD2, 0x61];

        b.iter(|| {
            GUID::build_from_components(d1, d2, d3, &d4);
        })
    }

    #[bench]
    fn bench_format(b: &mut Bencher) {
        let s = GUID::rand().to_string();
        b.iter(|| {
            format!("{}", GUID::parse(&s).unwrap());
        })
    }
}
