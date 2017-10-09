extern crate test;
extern crate logian;

use self::logian::file;

#[bench]
fn bench_read(b: &mut test::Bencher) {
    assert!(file::read("benches/feature/test.txt").is_ok());
    b.iter(|| file::read("benches/feature/test.txt"))
}

#[bench]
fn bench_read_glob(b: &mut test::Bencher) {
    assert!(file::read("benches/feature/test.txt").is_ok());
    b.iter(|| file::read_glob("benches/feature/*.txt"))
}
