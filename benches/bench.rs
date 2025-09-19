use divan::Bencher;
use guid_create::GUID;

fn main() {
    divan::main();
}

#[divan::bench]
#[cfg(feature = "rand")]
fn bench_rand(b: Bencher) {
    b.bench(|| {
        GUID::rand();
    });
}

#[divan::bench]
#[cfg(feature = "rand")]
fn bench_parse(b: Bencher) {
    let guid = GUID::rand();
    let s = guid.to_string();
    b.bench(|| {
        GUID::parse(&s).unwrap();
    })
}

#[divan::bench]
fn bench_build_from_array(b: Bencher) {
    let slice = [
        0x87, 0x93, 0x5C, 0xDE, 0x70, 0x94, 0x4C, 0x2B, 0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D, 0xD2,
        0x61,
    ];

    b.bench(|| {
        GUID::build_from_slice(&slice);
    })
}

#[divan::bench]
fn bench_build_from_components(b: Bencher) {
    let d1 = 0x87935CDE;
    let d2 = 0x7094;
    let d3 = 0x4C2B;
    let d4 = [0xA0, 0xF4, 0xDD, 0x7D, 0x51, 0x2D, 0xD2, 0x61];

    b.bench(|| {
        GUID::build_from_components(d1, d2, d3, &d4);
    })
}

#[divan::bench]
#[cfg(feature = "rand")]
fn bench_format(b: Bencher) {
    let s = GUID::rand().to_string();
    b.bench(|| {
        format!("{}", GUID::parse(&s).unwrap());
    })
}
