use crate::domain::entities::wallet::Wallet;

mod domain;

#[cfg(test)]
mod test_helpers;

fn main() {
    println!("Hello, world!");
    let wallet = Wallet::generate();
}
