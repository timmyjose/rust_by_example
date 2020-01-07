extern crate clap;

use clap::App;

fn main() {
    App::new("myapp")
        .version("1.0.0")
        .about("A new CLI")
        .author("Zoltan")
        .get_matches();
}
