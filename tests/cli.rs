use assert_cmd::prelude::*;
use assert_fs::TempDir;
use predicates::prelude::*;
use std::fs;
use std::process::Command;

fn command_with_config(temp_dir: &TempDir) -> Result<Command, Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("findme")?;
    cmd.env("FINDME_CONFIG_DIR", temp_dir.path());
    Ok(cmd)
}

#[test]
fn test_basic_fortune_output_english() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let mut cmd = command_with_config(&temp_dir)?;

    cmd.arg("--language").arg("en");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Overall Score"))
        .stdout(predicate::str::contains("Tech Advice"));

    temp_dir.close()?;
    Ok(())
}

#[test]
fn test_basic_fortune_output_chinese() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let mut cmd = command_with_config(&temp_dir)?;

    cmd.arg("--language").arg("zh");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("综合评分"))
        .stdout(predicate::str::contains("技术建议"));

    temp_dir.close()?;
    Ok(())
}

#[test]
fn test_verbose_output_english() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let mut cmd = command_with_config(&temp_dir)?;

    cmd.arg("--verbose").arg("--language").arg("en");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Findme"))
        .stdout(predicate::str::contains(
            "Developer's Daily Decompression Oracle",
        ));

    temp_dir.close()?;
    Ok(())
}

#[test]
fn test_verbose_output_chinese() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let mut cmd = command_with_config(&temp_dir)?;

    cmd.arg("--verbose").arg("--language").arg("zh");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Findme"))
        .stdout(predicate::str::contains("程序员今日解压占卜"));

    temp_dir.close()?;
    Ok(())
}

#[test]
fn test_language_option_english() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let mut cmd = command_with_config(&temp_dir)?;

    cmd.arg("--language").arg("en");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Overall Score"))
        .stdout(predicate::str::contains("Tech Advice"));

    temp_dir.close()?;
    Ok(())
}

#[test]
fn test_language_option_chinese() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let mut cmd = command_with_config(&temp_dir)?;

    cmd.arg("--language").arg("zh");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("综合评分"))
        .stdout(predicate::str::contains("技术建议"));

    temp_dir.close()?;
    Ok(())
}

#[test]
fn test_invalid_language() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let mut cmd = command_with_config(&temp_dir)?;

    cmd.arg("--language").arg("invalid");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid language option"));

    temp_dir.close()?;
    Ok(())
}

#[test]
fn test_help_output() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let mut cmd = command_with_config(&temp_dir)?;

    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("findme"))
        .stdout(predicate::str::contains(
            "Developer's Daily Decompression Oracle",
        ));

    temp_dir.close()?;
    Ok(())
}

#[test]
fn test_birthday_personalization_english() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let mut cmd = command_with_config(&temp_dir)?;

    cmd.arg("--birthday")
        .arg("1990-05-15")
        .arg("--language")
        .arg("en");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Overall Score"))
        .stdout(predicate::str::contains("Tech Advice"));

    temp_dir.close()?;
    Ok(())
}

#[test]
fn test_birthday_personalization_chinese() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let mut cmd = command_with_config(&temp_dir)?;

    cmd.arg("--birthday")
        .arg("1990-05-15")
        .arg("--language")
        .arg("zh");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("综合评分"))
        .stdout(predicate::str::contains("技术建议"));

    temp_dir.close()?;
    Ok(())
}

#[test]
fn test_birthday_invalid_date() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let mut cmd = command_with_config(&temp_dir)?;

    cmd.arg("--birthday")
        .arg("invalid-date")
        .arg("--language")
        .arg("en");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid birthday format"));

    temp_dir.close()?;
    Ok(())
}

#[test]
fn test_birthday_is_cached_in_config() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let mut cmd = command_with_config(&temp_dir)?;

    cmd.arg("--birthday")
        .arg("1990-05-15")
        .arg("--language")
        .arg("en");
    cmd.assert().success();

    let config_contents = fs::read_to_string(temp_dir.path().join("config.txt"))?;
    assert!(config_contents.contains("birthday=1990-05-15"));

    temp_dir.close()?;
    Ok(())
}

#[test]
fn test_cached_birthday_used_when_not_provided() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;

    let output_with_birthday = {
        let mut cmd = command_with_config(&temp_dir)?;
        cmd.arg("--birthday")
            .arg("1990-05-15")
            .arg("--language")
            .arg("en");
        cmd.output()?
    };
    assert!(output_with_birthday.status.success());
    let stdout_with_birthday = String::from_utf8(output_with_birthday.stdout)?;

    let output_with_cache = {
        let mut cmd = command_with_config(&temp_dir)?;
        cmd.arg("--language").arg("en");
        cmd.output()?
    };
    assert!(output_with_cache.status.success());
    let stdout_with_cache = String::from_utf8(output_with_cache.stdout)?;

    assert_eq!(stdout_with_birthday, stdout_with_cache);

    temp_dir.close()?;
    Ok(())
}
