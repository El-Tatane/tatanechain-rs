use crate::application::use_cases::generate_wallet::GenerateWalletUseCase;
use crate::application::use_cases::import_wallet_from_private_key::{ImportWalletFromPrivateKeyRequest, ImportWalletFromPrivateKeyUseCase};
use crate::infrastructure::crypto::k256_wallet_factory::K256WalletFactory;
mod domain;
mod application;
mod infrastructure;
use std::error::Error;

#[cfg(test)]
mod test_helpers;

fn main() -> Result<(), Box<dyn Error>>{
    println!("Welcome to TataneChain RS");

    let generate_wallet_use_case = GenerateWalletUseCase::new(K256WalletFactory{});
    let importer_wallet_use_case = ImportWalletFromPrivateKeyUseCase::new(K256WalletFactory{});


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

    Ok(())
}

