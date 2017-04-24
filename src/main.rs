// #[link(name = "boost_system", kind= "static")] // this works fine
#[link(name = "boost_system")]
extern "C" {
    fn print_time();
}

fn main() {
    unsafe{ print_time(); }
}
