pub struct Logger {}

impl Logger {
  pub fn info(label: &str, value: &str) {
    println!("->> {:<12} - {value}", label);
  }
}
