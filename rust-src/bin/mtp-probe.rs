extern crate libusb;
#[macro_use]
extern crate clap;
extern crate mtp;

use clap::App;
use clap::Arg;

fn main() {
    let matches = App::new("MTP Probe")
        .version("1.0")
        .author("Jean Pierre Dudey <jeandudey@hotmail.com>")
        .about("Probes if devices are MTP devices")
        .arg(Arg::with_name("bus")
            .short("b")
            .long("bus")
            .value_name("BUS")
            .help("The bus of the device to test")
            .takes_value(true))
        .arg(Arg::with_name("address")
            .short("a")
            .long("address")
            .value_name("ADDRESS")
            .help("The address of the device to test")
            .takes_value(true))
        .get_matches();

    let context = mtp::Context::new();

    let bus_number = match value_t!(matches, "bus", u8) {
        Ok(bus) => bus,
        Err(_) => {
            println!("Error: Not valid bus number.");
            return;
        }
    };

    let address = match value_t!(matches, "address", u8) {
        Ok(address) => address,
        Err(_) => {
            println!("Error: Not valid address number.");
            return;
        }
    };

    let is_mtp = mtp::probe::check_device(&context, bus_number, address);

    if is_mtp {
        println!("It's an MTP device");
    } else {
        println!("It isn't an MTP device");
    }
}
