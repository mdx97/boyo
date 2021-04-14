mod gb;
use clap::{App, Arg};
use gb::{Gameboy, Cartridge};

fn main() {
    let matches = App::new("boyo")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Mathew Horner <mathewhorner456@gmail.com>")
        .about("A cycle-accurate, efficient, and memory safe emulator for the Gameboy and Gameboy Advance.")
        .arg(Arg::with_name("rom")
            .required(true)
            .index(1))
        .get_matches();

    let path = matches.value_of("rom").unwrap();
    let mut gameboy = Gameboy::new(Cartridge::from(path).unwrap());
    // #[allow(while_true)]
    // while true {
    for _ in 0..32 {
        gameboy.tick();
    }
    //}
}