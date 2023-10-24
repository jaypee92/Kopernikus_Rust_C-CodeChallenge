// build.rs
fn main() {
    cc::Build::new().file("kpns.c").compile("kpns");
}