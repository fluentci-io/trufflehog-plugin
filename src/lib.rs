use extism_pdk::*;
use fluentci_pdk::dag;

#[plugin_fn]
pub fn setup(version: String) -> FnResult<String> {
    let version = if version.is_empty() {
        "latest".into()
    } else {
        format!("{}", version)
    };

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "install", &format!("trufflehog@{}", version)])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn git(flags: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "trufflehog", "git", &flags])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn github(flags: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "trufflehog", "github", &flags])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn gitlab(flags: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "trufflehog", "gitlab", &flags])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn filesystem(flags: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "trufflehog", "filesystem", &flags])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn s3(flags: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "trufflehog", "s3", &flags])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn gcs(flags: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "trufflehog", "gcs", &flags])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn syslog(flags: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "trufflehog", "syslog", &flags])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn circleci(flags: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "trufflehog", "circleci", &flags])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn docker(flags: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "trufflehog", "docker", &flags])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn travisci(flags: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "trufflehog", "travisci", &flags])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn postman(flags: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "trufflehog", "postman", &flags])?
        .stdout()?;
    Ok(stdout)
}
