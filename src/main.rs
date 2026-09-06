use crate::application::use_cases::generate_wallet::GenerateWalletUseCase;
use crate::application::use_cases::import_wallet_from_private_key::{ImportWalletFromPrivateKeyRequest, ImportWalletFromPrivateKeyUseCase};
use crate::infrastructure::crypto::k256_wallet_factory::K256WalletFactory;
mod domain;
mod application;
mod infrastructure;
use std::error::Error;
use crate::application::types::private_key::PrivateKey;
use crate::application::use_cases::create_transaction::CreateTransactionRequest;
use crate::application::use_cases::create_transaction::use_case::CreateTransactionUseCase;
use crate::domain::value_objects::address::Address;
use crate::domain::value_objects::public_key::PublicKey;
use crate::infrastructure::crypto::k256Signer::K256Signer;

#[cfg(test)]
mod test_helpers;

fn main() -> Result<(), Box<dyn Error>>{
    println!("Welcome to TataneChain RS");

    let generate_wallet_use_case = GenerateWalletUseCase::new(K256WalletFactory{});
    let importer_wallet_use_case = ImportWalletFromPrivateKeyUseCase::new(K256WalletFactory{});
    let create_transaction_use_case = CreateTransactionUseCase::new(K256WalletFactory{}, K256Signer {});

    let generated_wallet_response = generate_wallet_use_case.execute();

    println!(
        "new random identifiant created\n- address: {}\n- public_key: {}\n- private_key: {}\n",
        generated_wallet_response.address,
        generated_wallet_response.public_key,
        hex::encode(generated_wallet_response.private_key.reveal()),
    );


    let importer_wallet_request = ImportWalletFromPrivateKeyRequest{private_key: generated_wallet_response.private_key};
    let imported_wallet_response= importer_wallet_use_case.execute(importer_wallet_request)?;
    println!(
        "Imported private: created\n- address: {}\n- public_key: {}\n",
        imported_wallet_response.address,
        imported_wallet_response.public_key,
    );

    let private_key_alice = PrivateKey::new(
        hex::decode("a0ac60047ab55ac7733e2a7d6bfaeac635fb09936fca960cfeb477dfb6a10c80")
            .expect("valid hex")
            .try_into()
            .expect("32 bytes"),
    );

    let private_key_bob = PrivateKey::new(
        hex::decode("3d4e22e768737a9bee141c5d6a39cd0fcf6e15cf688c16bfb3914206f6357051")
        .expect("valid hex")
        .try_into()
        .expect("32 bytes"),
    );

    let public_key_bob = PublicKey::new(
        hex::decode("03baf49950f9c5a702c7d5671c51b82cc076c3b823f66a333b38027a53a9d4e532")
            .expect("valid hex")
            .try_into()
            .expect("33 bytes"),
    );

    let address_bob = Address::from_public_key(&public_key_bob);

    let create_transaction_request = CreateTransactionRequest{private_key: private_key_alice, recipient: address_bob, amount: 100};
    let create_transaction_response = create_transaction_use_case.execute(create_transaction_request)?;

    println!("Alice send 10O token to BOB :\n {}", create_transaction_response);

    Ok(())
}

