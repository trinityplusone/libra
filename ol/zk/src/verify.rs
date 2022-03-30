//! verify the proof

use std::{path::PathBuf, fs::File};
use leo_package::{outputs::VerificationKeyFile};
use snarkvm_algorithms::{
    snark::groth16::{Groth16, PreparedVerifyingKey, VerifyingKey, Proof},
    traits::snark::SNARK,
};
use snarkvm_curves::bls12_377::{Bls12_377, Fr, Bls12_377Parameters};
use snarkvm_curves::templates::bls12::Bls12;

use leo_compiler::{compiler::Compiler, group::targets::edwards_bls12::EdwardsGroupType};

/// do the verification
pub fn verify() {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let p = p.parent().unwrap().join("zk/hello-world");
    dbg!(&p);

    //  match InputPairs::try_from(p.as_path()) {
    //     Ok(pairs) => {
          
    //       // dbg!(pairs.pairs);
    //     },
    //     Err(_) => {
    //         dbg!("err");
    //     }
    // };

    let package_name = "demo";
    let verifying_key_bytes = VerificationKeyFile::new(&package_name)
        .read_from(&p.join("outputs/hello-world.lvk")).unwrap();
        // .map_err(CliError::cli_io_error).unwrap();
    // dbg!(&verifying_key_bytes);
    let verifying_key =
        VerifyingKey::<Bls12_377>::read(verifying_key_bytes.as_slice()).unwrap();

    // // Derive the prepared verifying key file from the verifying key
    let prepared_verifying_key = PreparedVerifyingKey::<Bls12_377>::from(verifying_key);

    // dbg!(&prepared_verifying_key);

    let f = File::open(&p.join("outputs/hello-world.proof")).unwrap();
    // let mut buffer: Vec<u8> = vec![];
    // // read up to 10 bytes
    // f.read_to_end(&mut buffer).unwrap();
    // // (&mut data);
    // dbg!(buffer);

    // Read::
    // Proof::try_from(buffer.as_slice());

    let proof = Proof::<Bls12<Bls12_377Parameters>>::read(f).unwrap();

    dbg!(&proof);
    // Proof<Bls12<Bls12_377Parameters>>
    // let proof_file = match ProofFile::new(package_name).read_from(&p.join("outputs/hello-world.proof")) {
    //     Ok(a) => a,
    //     Err(e) => { 
    //       dbg!(&e);
    //       exit(1)
    //     },
    // };

    let is_success = Groth16::<Bls12_377, Compiler<'_, Fr, EdwardsGroupType>, Vec<Fr>>::verify(
      &prepared_verifying_key,
      &vec![],
      &proof,
    );

    dbg!(is_success);
}
