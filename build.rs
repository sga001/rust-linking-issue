extern crate gcc;

fn main() {
    gcc::Config::new()
                .file("src/cpp/time.cpp")
                .include("src/cpp/")
                .cpp(true)
                .compile("libtest_time.a");
}
