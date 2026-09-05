use crate::application::use_cases::generate_wallet::GenerateWalletUseCase;
use crate::infrastructure::crypto::k256_wallet_generator::K256WalletGenerator;
mod domain;
mod application;
mod infrastructure;

#[cfg(test)]
mod test_helpers;

fn main() {
    println!("Welcome to TataneChain RS");
    let wallet_generator = K256WalletGenerator{};

    let generate_wallet_use_case = GenerateWalletUseCase::new(wallet_generator);

    let wallet_response = generate_wallet_use_case.execute();

    println!(
        "address: {}\npublic_key: {}\nprivate_key: {}",
        wallet_response.address,
        wallet_response.public_key,
        hex::encode(wallet_response.private_key.reveal()),
    );
}

