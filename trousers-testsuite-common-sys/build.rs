extern crate gcc;

fn main() {
    gcc::Config::new()
        .file("trousers-testsuite/tcg/common/common.c")
        .include("trousers-testsuite/tcg/include")
        .compile("libtrousers-testsuite-common.a");
}
