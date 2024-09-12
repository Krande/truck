use truck_stepio::r#in::Table;
const TEMP_DIRECTORY: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../temp/");

#[test]
fn io_big() {
    let step_file = [TEMP_DIRECTORY, "big.stp"].concat();
    let step_string = std::fs::read_to_string(step_file).unwrap();
    let table = Table::from_step(&step_string).unwrap();
}