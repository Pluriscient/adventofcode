
#[macro_use]
extern crate lazy_static;

#[allow(dead_code)] // disable warning over dead code
mod years;

fn main() {
    println!("Hello, world!");
}

// #[cfg(test)]
mod test {
    #[test]
    fn run() {
        println!("running test");
    }
}