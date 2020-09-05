use std::io::{self, Read};
use structopt::StructOpt;

/*
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn fail() {
        panic!("This failed");
    }
}
*/

#[derive(Debug, StructOpt)]
#[structopt(name = "Abbreviate", about = "Abv-rs options")]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: bool,

    /// Set lenght of the output
    #[structopt(short, long, default_value = "7")]
    lenght: usize,
}

fn main() {
    // command line options
    let opt = Opt::from_args();

    let mut buffer: String = String::new();
    let stdin = io::stdin();

    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer).unwrap();

    if opt.debug == true {
        println!("The current limit is: {}", opt.lenght);
        println!("The Stdin is: {}", buffer);
    }

    // actual string manipulation
    let input: Vec<&str> = buffer.split("\n").collect();
    let mut input: String = input[0].to_string();

    let (input, useless) = input.split_at(opt.lenght);

    if opt.debug == true {
        println!("The input after the split: {}", input);
    }


    let output = format!("{}...", input);


    print!("{}", output);

}