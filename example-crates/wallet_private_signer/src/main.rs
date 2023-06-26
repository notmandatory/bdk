use std::{io::Write, str::FromStr};
use std::collections::BTreeMap;

use bdk::bitcoin::secp256k1::{All, PublicKey};
use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::util::bip32::{DerivationPath, ExtendedPrivKey, KeySource};
use bdk::descriptor::DescriptorPublicKey;
use bdk::keys::DescriptorKey::Secret;
use bdk::keys::DescriptorSecretKey::XPrv;
use bdk::keys::{DerivableKey, DescriptorKey, DescriptorSecretKey};
use bdk::miniscript::psbt::PsbtExt;
use bdk::miniscript::Segwitv0;
use bdk::signer::{InputSigner, SignerContext, SignerWrapper};
use bdk::{
    bitcoin::{Address, Network},
    descriptor,
    wallet::AddressIndex,
    Error, SignOptions, Wallet,
};
use bdk::bitcoin::psbt::PartiallySignedTransaction;
use bdk::bitcoin::util::taproot::TapLeafHash;
use bdk::bitcoin::XOnlyPublicKey;
use bdk_esplora::{esplora_client, EsploraAsyncExt};
use bdk_file_store::Store;

const DB_MAGIC: &str = "bdk_wallet_private_signer_example";
const STOP_GAP: usize = 50;
const PARALLEL_REQUESTS: usize = 5;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secp = Secp256k1::new();
    let db_path = std::env::temp_dir().join("bdk_wallet_private_signer_example");
    dbg!(&db_path);
    let db = Store::<bdk::wallet::ChangeSet>::new_from_path(DB_MAGIC.as_bytes(), db_path)?;

    // TODO make sure to generate this is provably unspendable, see: https://github.com/Coldcard/firmware/blob/edge/docs/taproot.md#provably-unspendable-internal-key
    let unspendable_key = DescriptorPublicKey::from_str(
        "020000000000000000000000000000000000000000000000000000000000000001",
    )?;

    let app_hardened_path =
        DerivationPath::from_str("m/86h/1h/0h").expect("mobile app hardened path");
    let svr_hardened_path =
        DerivationPath::from_str("m/86h/1h/100h").expect("signing server hardened path");
    let hws_hardened_path =
        DerivationPath::from_str("m/86h/1h/0h").expect("hardware signer hardened path");

    let ext_unhardened_path = DerivationPath::from_str("m/0").expect("external unhardened path");
    let int_unhardened_path = DerivationPath::from_str("m/1").expect("internal unhardened path");

    let app_xprv = ExtendedPrivKey::from_str("tprv8ZgxMBicQKsPedrWwpHU4mTithdAR2zrFSww4aEQcruu1ZQ4rkchBPiUSqi6Bs1pVNitDViE5LenmTE5GxeHyP8qdBGXqvGPDLa6ffDHMJo").expect("alice1 xprv");

    let (app_ext_desc_xprv_0, _app_ext_desc_xpub_0, _app_ext_keysource_0) = derive_descriptor_keys(
        &secp,
        app_xprv.clone(),
        app_hardened_path.clone(),
        ext_unhardened_path.clone(),
    )
    .expect("mobile app external descriptor keys");
    dbg!(&_app_ext_desc_xpub_0.to_string());

    let (app_ext_desc_xprv_1, _app_ext_desc_xpub_1, _app_ext_keysource_1) = derive_descriptor_keys(
        &secp,
        app_xprv.clone(),
        app_hardened_path.clone(),
        ext_unhardened_path.clone(),
    )
    .expect("mobile app external descriptor keys");

    let (app_int_desc_xprv_0, _app_int_desc_xpub_0, _app_int_keysource_0) = derive_descriptor_keys(
        &secp,
        app_xprv.clone(),
        app_hardened_path.clone(),
        int_unhardened_path.clone(),
    )
    .expect("mobile app internal descriptor keys");

    let (app_int_desc_xprv_1, _app_int_desc_xpub_1, _app_int_keysource_1) = derive_descriptor_keys(
        &secp,
        app_xprv.clone(),
        app_hardened_path.clone(),
        int_unhardened_path.clone(),
    )
    .expect("mobile app internal descriptor keys");

    let svr_xprv = ExtendedPrivKey::from_str("tprv8ZgxMBicQKsPdN9ucBASBhLRFVGJuK16SRLgDmHWn9wwXpd9xBvdwAmTkMQqpNd2jA5gyNDCdQUXa5QY8pUex2SPjoQgbPh8QFZnJNhUqwp").expect("alice1 xprv");

    let (svr_ext_desc_xprv, svr_ext_desc_xpub, svr_ext_keysource) = derive_descriptor_keys(
        &secp,
        svr_xprv.clone(),
        svr_hardened_path.clone(),
        ext_unhardened_path.clone(),
    )
    .expect("signing server external descriptor keys");
    dbg!(&svr_ext_desc_xpub.to_string());

    let (svr_int_desc_xprv, svr_int_desc_xpub, _svr_int_keysource) = derive_descriptor_keys(
        &secp,
        svr_xprv,
        svr_hardened_path.clone(),
        int_unhardened_path.clone(),
    )
    .expect("signing server internal descriptor keys");

    let hws_xprv = ExtendedPrivKey::from_str("tprv8ZgxMBicQKsPdjLy9KVsCEeD7bgtrDpVeALuiHipxTwTQCyJChMcpqnbemEWGdWBbSbmFmhzGiDvzp1T3uyycdyTfZBtMFpkQAkCsZeZ535").expect("alice1 xprv");

    let (_hws_ext_desc_xprv, hws_ext_desc_xpub, _hws_ext_keysource) = derive_descriptor_keys(
        &secp,
        hws_xprv.clone(),
        hws_hardened_path.clone(),
        ext_unhardened_path.clone(),
    )
    .expect("hardware signer external descriptor keys");
    dbg!(&hws_ext_desc_xpub.to_string());

    let (_hws_int_desc_xprv, hws_int_desc_xpub, _hws_int_keysource) = derive_descriptor_keys(
        &secp,
        hws_xprv,
        hws_hardened_path.clone(),
        int_unhardened_path.clone(),
    )
    .expect("hardware signer internal descriptor keys");

    // and_v(v:pk(APP),pk(SVR)); and_v(v:pk(APP),pk(HWS)); and_v(v:pk(HWS),pk(SVR))

    let (ext_descriptor, ext_key_map, _ext_networks) =
        descriptor!(tr(unspendable_key.clone(), { and_v(v:pk(app_ext_desc_xprv_0),pk(svr_ext_desc_xpub.clone())),{and_v(v:pk(app_ext_desc_xprv_1),pk(hws_ext_desc_xpub.clone())),and_v(v:pk(hws_ext_desc_xpub),pk(svr_ext_desc_xpub))}})).expect("external descriptor");
    dbg!(&ext_descriptor.to_string_with_secret(&ext_key_map));

    let (int_descriptor, int_key_map, _int_networks) =
        descriptor!(tr(unspendable_key, { and_v(v:pk(app_int_desc_xprv_0),pk(svr_int_desc_xpub.clone())),{and_v(v:pk(app_int_desc_xprv_1),pk(hws_int_desc_xpub.clone())),and_v(v:pk(hws_int_desc_xpub),pk(svr_int_desc_xpub))}})).expect("internal descriptor");
    dbg!(&int_descriptor.to_string_with_secret(&int_key_map));

    let mut app_wallet = Wallet::new(
        (ext_descriptor, ext_key_map),
        Some((int_descriptor, int_key_map)),
        db,
        Network::Signet,
    )
    .expect("app wallet");

    let balance = app_wallet.get_balance();
    println!(
        "Wallet confirmed balance before syncing: {} sats",
        balance.confirmed
    );

    print!("Syncing...");
    let client = esplora_client::Builder::new("https://mutinynet.com/api").build_async()?;

    let local_chain = app_wallet.checkpoints();
    let keychain_spks = app_wallet
        .spks_of_all_keychains()
        .into_iter()
        .map(|(k, k_spks)| {
            let mut once = Some(());
            let mut stdout = std::io::stdout();
            let k_spks = k_spks
                .inspect(move |(spk_i, _)| match once.take() {
                    Some(_) => print!("\nScanning keychain [{:?}]", k),
                    None => print!(" {:<3}", spk_i),
                })
                .inspect(move |_| stdout.flush().expect("must flush"));
            (k, k_spks)
        })
        .collect();

    let update = client
        .scan(
            local_chain,
            keychain_spks,
            [],
            [],
            STOP_GAP,
            PARALLEL_REQUESTS,
        )
        .await?;
    println!("After scan...");
    app_wallet.apply_update(update)?;
    app_wallet.commit()?;
    println!("Update applied and committed.");

    let balance = app_wallet.get_balance();
    println!(
        "Wallet confirmed balance after syncing: {} sats",
        balance.confirmed
    );

    if balance.confirmed < 10_000 {
        let deposit_address = app_wallet.get_address(AddressIndex::New);
        //.expect("deposit address");  // TODO get_address should return a result
        println!(
            "Send at least 10,000 SATs (0.0001 BTC) from the u01.net testnet faucet to address '{addr}'.\nFaucet URL: https://faucet.mutinynet.com/",
            addr = deposit_address.address
        );
        return Ok(());
    }

    let faucet_address =
        Address::from_str("mkHS9ne12qx9pS9VojpwU5xtRd4T7X7ZUt").expect("faucet address");

    let mut app_tx_builder = app_wallet.build_tx();
    app_tx_builder.add_recipient(faucet_address.script_pubkey(), 3300);

    let (mut app_psbt, app_tx_details) = app_tx_builder.finish().expect("app created psbt");
    dbg!(&app_tx_details);

    // app signs the psbt
    let mut sign_options = SignOptions::default();
    sign_options.sign_with_tap_internal_key = false;

    let _is_signed = app_wallet
        .sign(&mut app_psbt, sign_options.clone())
        .expect("app signed psbt");
    dbg!(&app_psbt);

    // create server psbt with app and hws bip32 derivations filtered out
    let mut svr_psbt = app_psbt.clone();
    filter_bip32_derivation(&mut svr_psbt, svr_ext_keysource);
    // dbg!(&server_psbt.to_string());
    dbg!(&svr_psbt);

    // signing server signs filtered psbt with their ext and int descriptor xprv keys
    for svr_desc_xprv in [svr_ext_desc_xprv, svr_int_desc_xprv] {
        if let XPrv(server_xprv) = svr_desc_xprv {
            let signer_wrapper = SignerWrapper::new(
                server_xprv,
                SignerContext::Tap {
                    is_internal_key: false,
                },
            );
            for i in 0..svr_psbt.inputs.len() {
                signer_wrapper
                    .sign_input(&mut svr_psbt, i, &sign_options, &secp)
                    .expect("signed input");
            }
        }
    }

    dbg!(&svr_psbt.to_string());
    dbg!(&svr_psbt);
    let finalized_psbt = svr_psbt.finalize(&secp).expect("finalized psbt");
    dbg!(&finalized_psbt);
    let tx = &finalized_psbt.extract_tx();
    client.broadcast(tx).await.expect("transaction broadcast");
    println!("https://mutinynet.com/tx/{}", &tx.txid());

    Ok(())
}

fn derive_descriptor_keys(
    secp: &Secp256k1<All>,
    origin_xprv: ExtendedPrivKey,
    hardened_path: DerivationPath,
    unhardened_path: DerivationPath,
) -> Result<(DescriptorSecretKey, DescriptorPublicKey, KeySource), Error> {
    let derived_xprv = &origin_xprv.derive_priv(&secp, &hardened_path)?;
    let origin: KeySource = (origin_xprv.fingerprint(&secp), hardened_path);

    let derived_xprv_desc_key: DescriptorKey<Segwitv0> =
        derived_xprv.into_descriptor_key(Some(origin.clone()), unhardened_path.clone())?;

    if let Secret(desc_seckey, _, _) = derived_xprv_desc_key {
        let desc_pubkey = desc_seckey
            .to_public(&secp)
            .map_err(|e| Error::Generic(e.to_string()))?;

        Ok((desc_seckey, desc_pubkey, origin))
    } else {
        unreachable!()
    }
}

fn filter_bip32_derivation(psbt: &mut PartiallySignedTransaction, keep_keysource: KeySource) {
    // filter inputs to include only one bip32 derivation and tap key origins key source
    let mut filtered_inputs = psbt.inputs.clone();
    filtered_inputs.iter_mut().for_each(|i| {
        let filtered_bip32_derivation: BTreeMap<PublicKey, KeySource> = i
            .bip32_derivation
            .iter()
            .map(|d| (d.0.clone(), d.1.clone()))
            .filter(|d| d.1 .0 == keep_keysource.0)
            .collect();
        i.bip32_derivation = filtered_bip32_derivation;
        let filtered_tap_key_origins: BTreeMap<XOnlyPublicKey, (Vec<TapLeafHash>, KeySource)> = i
            .tap_key_origins
            .iter()
            .map(|d| (d.0.clone(), d.1.clone()))
            .filter(|d| d.1.1.0 == keep_keysource.0)
            .collect();
        i.tap_key_origins = filtered_tap_key_origins;
    });
    psbt.inputs = filtered_inputs;

    // filter outputs to include only one bip32 derivation and tap key origins key source
    let mut filtered_outputs = psbt.outputs.clone();
    filtered_outputs.iter_mut().for_each(|o| {
        let filtered_bip32_derivation: BTreeMap<PublicKey, KeySource> = o
            .bip32_derivation
            .iter()
            .map(|d| (d.0.clone(), d.1.clone()))
            .filter(|d| d.1.0 == keep_keysource.0)
            .collect();
        o.bip32_derivation = filtered_bip32_derivation;
        let filtered_tap_key_origins: BTreeMap<XOnlyPublicKey, (Vec<TapLeafHash>, KeySource)> = o
            .tap_key_origins
            .iter()
            .map(|d| (d.0.clone(), d.1.clone()))
            .filter(|d| d.1.1.0 == keep_keysource.0)
            .collect();
        o.tap_key_origins = filtered_tap_key_origins;
    });
    psbt.outputs = filtered_outputs;
}
