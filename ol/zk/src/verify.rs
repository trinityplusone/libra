//! verify the proof

use std::{path::PathBuf, fs::File};
use leo_package::{outputs::VerificationKeyFile};
use snarkvm_algorithms::{
    snark::groth16::{Groth16, PreparedVerifyingKey, VerifyingKey, Proof},
    traits::snark::SNARK,
};
use anyhow::Error;
use snarkvm_curves::bls12_377::{Bls12_377, Fr, Bls12_377Parameters};
use snarkvm_curves::templates::bls12::Bls12;

use leo_compiler::{compiler::Compiler, group::targets::edwards_bls12::EdwardsGroupType};

/// do the verification
pub fn verify() -> Result<bool, Error> {
  let package_name = "hello-world";

    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let p = p.join("fixtures/outputs");
    // dbg!(&p);

    let verify_file = &p.join(format!("{}.lvk", &package_name));
    // dbg!(&verify_file);

    let verifying_key_bytes = VerificationKeyFile::new(&package_name)
      .read_from(verify_file)?;

    let verifying_key =
        VerifyingKey::<Bls12_377>::read(verifying_key_bytes.as_slice())?;

    let prepared_verifying_key = PreparedVerifyingKey::<Bls12_377>::from(verifying_key);

    // dbg!(&prepared_verifying_key);
    
    let proof_file = &p.join(format!("{}.proof", &package_name));
    let f = File::open(&proof_file)?;

    let proof = Proof::<Bls12<Bls12_377Parameters>>::read(f)?;

    let is_success = Groth16::<Bls12_377, Compiler<'_, Fr, EdwardsGroupType>, Vec<Fr>>::verify(
      &prepared_verifying_key,
      &vec![],
      &proof,
    )?;

    dbg!(is_success);
    Ok(is_success)
}
