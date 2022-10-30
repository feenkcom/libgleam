use std::error::Error;

use shared_library_builder::build_standalone;

use libgleam_library::latest_libgleam;

fn main() -> Result<(), Box<dyn Error>> {
    build_standalone(|_| Ok(Box::new(latest_libgleam())))
}
