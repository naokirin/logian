extern crate test;
extern crate logian;

use self::logian::file;

#[bench]
fn bench_read(b: &mut test::Bencher) {
    assert!(file::reader::read("benches/feature/test.txt").is_ok());
    b.iter(|| file::reader::read("benches/feature/test.txt"))
}

#[bench]
fn bench_read_glob(b: &mut test::Bencher) {
    assert!(file::reader::read("benches/feature/test.txt").is_ok());
    b.iter(|| file::reader::read_glob("benches/feature/*.txt"))
}
