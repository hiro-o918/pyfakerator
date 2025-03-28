use fakerator_core::generator::write_factory_codes;
fn main() {
    let root_dir = std::env::current_dir().unwrap().join("resources/src");
    write_factory_codes(&root_dir).unwrap();
}
