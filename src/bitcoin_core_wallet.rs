use bdk_wallet::bitcoin::secp256k1::rand::{self, RngCore};
use bdk_wallet::bitcoin::{Network, bip32::Xpriv, NetworkKind};
use bdk_wallet::{KeychainKind, template::{Bip86, Bip44, Bip49, Bip84, DescriptorTemplate}, KeyRing};

fn get_random_bytes() -> [u8; 32] {
    let mut seed: [u8; 32] = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut seed);
    seed
}

#[derive(PartialOrd, PartialEq, Eq, Ord, Clone, Debug)]
pub enum CoreKeychain {
    PkhExternal,
    PkhInternal,
    ShExternal,
    ShInternal,
    WpkhExternal,
    WpkhInternal,
    TrExternal,
    TrInternal
}

impl CoreKeychain {
    pub fn is_internal(&self) -> bool {
        match self {
            CoreKeychain::TrInternal
            | CoreKeychain::WpkhInternal
            | CoreKeychain::PkhInternal
            | CoreKeychain::ShInternal => true,
            _ => false,
        }
    }
}


fn main() {
    let network: Network = Network::Signet;
    let mut keyring = KeyRing::new(network);

    let xprv: Xpriv = Xpriv::new_master(network, &get_random_bytes()).unwrap();
    
    // Add the Bip44 descriptors
    let (descriptor, _, _) = Bip44(xprv, KeychainKind::External)
        .build(NetworkKind::Test)
        .expect("Failed to build external descriptor");
    
    let (change_descriptor, _, _) = Bip44(xprv, KeychainKind::Internal)
        .build(NetworkKind::Test)
        .expect("Failed to build internal descriptor");

    keyring.add_descriptor(CoreKeychain::PkhExternal, descriptor).unwrap();
    keyring.add_descriptor(CoreKeychain::PkhInternal, change_descriptor).unwrap();

    // Add the Bip49 descriptors
    let (descriptor, _, _) = Bip49(xprv, KeychainKind::External)
        .build(NetworkKind::Test)
        .expect("Failed to build external descriptor");
    
    let (change_descriptor, _, _) = Bip49(xprv.clone(), KeychainKind::Internal)
        .build(NetworkKind::Test)
        .expect("Failed to build internal descriptor");

    keyring.add_descriptor(CoreKeychain::ShExternal, descriptor).unwrap();
    keyring.add_descriptor(CoreKeychain::ShInternal, change_descriptor).unwrap();

    // Add the Bip84 descriptors
    let (descriptor, _, _) = Bip84(xprv, KeychainKind::External)
        .build(NetworkKind::Test)
        .expect("Failed to build external descriptor");
    
    let (change_descriptor, _, _) = Bip84(xprv.clone(), KeychainKind::Internal)
        .build(NetworkKind::Test)
        .expect("Failed to build internal descriptor");

    keyring.add_descriptor(CoreKeychain::WpkhExternal, descriptor).unwrap();
    keyring.add_descriptor(CoreKeychain::WpkhInternal, change_descriptor).unwrap();

    // Add the Bip86 descriptors
    let (descriptor, _, _) = Bip86(xprv, KeychainKind::External)
        .build(NetworkKind::Test)
        .expect("Failed to build external descriptor");
    
    let (change_descriptor, _, _) = Bip86(xprv.clone(), KeychainKind::Internal)
        .build(NetworkKind::Test)
        .expect("Failed to build internal descriptor");

    keyring.add_descriptor(CoreKeychain::TrExternal, descriptor).unwrap();
    keyring.add_descriptor(CoreKeychain::TrInternal, change_descriptor).unwrap();


    let wallet = keyring.into_params().unwrap().create_wallet_no_persist().unwrap();

    println!("{{\n  \"name\": \"bdk_wallet\"\n}}");

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let descriptors = wallet.keychains();

    println!("{{\n  \"wallet_name\": \"bdk_wallet\",\n  \"descriptors\": [");
    let mut first = true;
    for (keychain, descriptor) in descriptors {
        if !first {
            println!(",");
        }
        first = false;
        let next_index = wallet.next_derivation_index(keychain.clone()).unwrap();
        let internal = keychain.is_internal();
        print!(
            "    {{\n      \"desc\": \"{descriptor}\",\n      \"timestamp\": {timestamp},\n      \"active\": true,\n      \"internal\": {internal},\n      \"next\": {next_index},\n      \"next_index\": {next_index}\n    }}"
        );
    }
    println!("\n  ]\n}}");
}
