use anyhow::Result;
use assert_cmd::Command;
use assert_fs::prelude::*;
use assert_fs::NamedTempFile;
use arboard::Clipboard;
use paster::{
    config::{DestinationConfig, PasterConfig},
    paste::debug::DebugConfig,
};
use predicates::str::contains;

use std::collections::HashMap;


fn create_test_config() -> Result<NamedTempFile> {
    let mut dest = HashMap::new();
    dest.insert("test".to_owned(), DestinationConfig::Debug(DebugConfig));

    let config = PasterConfig {
        version: "1".to_owned(),
        default: "test".to_owned(),
        dest,
    };
    let config_file = NamedTempFile::new("config.yaml")?;
    confy::store_path(config_file.path(), config)?;

    Ok(config_file)
}

fn create_test_cmd(config_file: &NamedTempFile) -> Result<Command> {
    let mut cmd = Command::cargo_bin("paster")?;
    cmd.args(["-c", config_file.to_str().unwrap()]);

    Ok(cmd)
}

#[test]
fn test_stdin_paste() -> Result<()> {
    let config_file = create_test_config()?;
    let mut cmd = create_test_cmd(&config_file)?;

    let test_paste = "Test stdin";
    cmd.write_stdin(test_paste)
        .assert()
        .success();

    let mut clipboard = Clipboard::new().unwrap();
    assert_eq!(test_paste, clipboard.get_text()?);

    Ok(())
}

#[test]
fn test_file_paste() -> Result<()> {
    let config_file = create_test_config()?;
    let mut cmd = create_test_cmd(&config_file)?;

    let test_file = NamedTempFile::new("test.txt")?;
    let test_paste = "Test file";
    test_file.write_str(test_paste)?;
    cmd.arg(test_file.to_str().unwrap())
        .assert()
        .success();

    let mut clipboard = Clipboard::new().unwrap();
    assert_eq!(test_paste, clipboard.get_text()?);

    Ok(())
}

#[test]
fn test_unknown_dest() -> Result<()> {
    let config_file = create_test_config()?;
    let mut cmd = create_test_cmd(&config_file)?;

    cmd.args(["-d", "unknown"])
        .assert()
        .failure()
        .stderr(contains("Unknown destination name"));

    Ok(())
}

#[test]
fn test_config_update() -> Result<()> {
    let config_file = create_test_config()?;
    let mut cmd = create_test_cmd(&config_file)?;

    cmd.args(["config", "default", "modified"])
        .assert()
        .success();

    config_file.assert(contains("default: modified"));

    Ok(())
}
