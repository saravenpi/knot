use knot::config::KnotConfig; fn main() { let c = KnotConfig::from_file(std::path::Path::new("knot.yml")); println!("{:?}", c); }
