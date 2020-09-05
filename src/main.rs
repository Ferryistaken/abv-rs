use std::io::{self, Read};

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

fn main() {
    let mut buffer: String = String::new();
    let stdin = io::stdin();

    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer).unwrap();

    print!("The Stdin is: {}", buffer);
}