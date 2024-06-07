use anyhow::Result;
use assert_cmd::Command;
use predicates::boolean::PredicateBooleanExt;

#[test]
fn dies_no_args() -> Result<()> {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert().stderr(predicates::str::contains("Usage"));
    predicates::str::contains("hello").and(predicates::iter::in_hash(vec!["1"]));
    Ok(())
}

#[test]
fn runs_with_success() -> Result<()> {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("hello").assert().success();
    Ok(())
}
