use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;
use helpers::setup_arkade;

#[plugin_fn]
pub fn setup(_args: String) -> FnResult<String> {
    let stdout = setup_arkade()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn chart(args: String) -> FnResult<String> {
    setup_arkade()?;
    let stdout = dag()
        .pipeline("chart")?
        .with_exec(vec!["arkade", "chart", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn get(args: String) -> FnResult<String> {
    let stdout = dag()
        .pipeline("get")?
        .with_exec(vec!["arkade", "get", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn install(args: String) -> FnResult<String> {
    let stdout = dag()
        .pipeline("install")?
        .with_exec(vec!["arkade", "install", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn oci(args: String) -> FnResult<String> {
    let stdout = dag()
        .pipeline("oci")?
        .with_exec(vec!["arkade", "oci", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn system(args: String) -> FnResult<String> {
    let stdout = dag()
        .pipeline("system")?
        .with_exec(vec!["arkade", "system", &args])?
        .stdout()?;
    Ok(stdout)
}
