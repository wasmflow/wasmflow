use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::{Path, PathBuf};
use std::{env, fs};

use nkeys::{KeyPair, KeyPairType};
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
pub(crate) struct GenerateCommon {
  /// Location of key files for signing. Defaults to $VINO_KEYS ($HOME/.vino/keys or %USERPROFILE%/.vino/keys on Windows).
  #[structopt(long = "directory", env = "VINO_KEYS", hide_env_values = true)]
  pub(crate) directory: Option<String>,

  /// Set the token expiration in days. By default the token will never expire.
  #[structopt(short = "x", long = "expires")]
  pub(crate) expires_in_days: Option<u64>,

  /// Period in days before token becomes valid. By default the token will be valid immediately.
  #[structopt(short = "b", long)]
  pub(crate) not_before: Option<u64>,
}

/// Retrieves a keypair by name in a specified directory, or $VINO_KEYS ($HOME/.vino/keys) if directory is not specified.
pub(crate) fn _get(keyname: &str, directory: Option<String>) -> Result<(), Box<dyn ::std::error::Error>> {
  let dir = determine_directory(directory)?;
  let mut f = File::open(format!("{}/{}", dir, keyname))
    .map_err(|e| format!("{}.\nPlease ensure {}/{} exists.", e, dir, keyname))?;

  let mut s = String::new();
  let res = match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
  };
  match res {
    Err(e) => Err(e.into()),
    Ok(s) => {
      debug!("{}", s.trim().to_owned());
      Ok(())
    }
  }
}

/// Lists all keypairs (file extension .nk) in a specified directory or $VINO_KEYS($HOME/.vino/keys) if directory is not specified.
pub(crate) fn _list(directory: Option<String>) -> Result<(), Box<dyn ::std::error::Error>> {
  let dir = determine_directory(directory)?;

  let mut keys = vec![];
  let paths = fs::read_dir(dir.clone()).map_err(|e| format!("Error: {}, please ensure directory {} exists", e, dir))?;

  for path in paths {
    let f = String::from(path.unwrap().file_name().to_str().unwrap());
    if f.ends_with(".nk") {
      keys.push(f);
    }
  }

  debug!("====== Keys found in {} ======\n{}", dir, keys.join("\n"));
  Ok(())
}

fn determine_directory(directory: Option<String>) -> Result<String, Error> {
  #[cfg(not(target_os = "windows"))]
  let env_home = "HOME";
  #[cfg(target_os = "windows")]
  let env_home = "USERPROFILE";

  if let Some(d) = directory {
    Ok(d)
  } else if let Ok(home) = env::var(env_home) {
    Ok(format!("{}/.vino/keys", home))
  } else {
    Err(Error::new(
      std::io::ErrorKind::NotFound,
      format!(
        "${} not found, please set ${} or $VINO_KEYS for autogenerated keys",
        env_home, env_home
      ),
    ))
  }
}

/// Helper function to locate and extract keypair from user input.
/// Returns a tuple of the keypair and optional autogenerate message.
pub(crate) fn extract_keypair(
  module_path: Option<String>,
  directory: Option<String>,
  keygen_type: KeyPairType,
) -> Result<KeyPair, crate::Error> {
  let seed = if let Some(module) = module_path {
    // No seed value provided, attempting to source from provided or default directory
    let dir = determine_directory(directory)?;
    // Account key should be re-used, and will attempt to generate based on the terminal USER
    let module_name = match keygen_type {
      KeyPairType::Account => std::env::var("USER").unwrap_or_else(|_| "user".to_owned()),
      _ => PathBuf::from(module).file_stem().unwrap().to_str().unwrap().to_owned(),
    };
    let path = format!("{}/{}_{}.nk", dir, module_name, keypair_type_to_string(&keygen_type));
    match File::open(path.clone()) {
      // Default key found
      Ok(mut f) => {
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        s
      }
      // No default key, generating for user
      Err(_e) => {
        info!("No keypair found in \"{}\", generating a new pair.", path);

        let kp = KeyPair::new(keygen_type);
        let seed = kp.seed()?;
        fs::create_dir_all(Path::new(&path).parent().unwrap())?;
        let mut f = File::create(path)?;
        f.write_all(seed.as_bytes())?;
        seed
      }
    }
  } else {
    return Err(crate::Error::KeyPairNotProvided);
  };

  Ok(KeyPair::from_seed(&seed)?)
}

fn keypair_type_to_string(keypair_type: &KeyPairType) -> String {
  use KeyPairType::*;
  match keypair_type {
    Account => "account".to_owned(),
    Cluster => "cluster".to_owned(),
    Service => "service".to_owned(),
    Module => "module".to_owned(),
    Server => "server".to_owned(),
    Operator => "operator".to_owned(),
    User => "user".to_owned(),
  }
}
