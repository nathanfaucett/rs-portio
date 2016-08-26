extern crate portio;


use portio::Portio;


fn handler(string: &str) {
    println!("{}", string);
}


fn main() {
    let portio = Portio::new(handler);
    portio.listen();
}
