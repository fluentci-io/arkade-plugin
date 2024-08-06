use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup_arkade() -> Result<String, Error> {
    let is_root = dag()
        .pkgx()?
        .with_exec(vec!["whoami"])?
        .stdout()?
        .contains("root");

    match is_root {
        true => dag().set_envs(vec![("SUDO".into(), "".into())])?,
        false => dag().set_envs(vec![("SUDO".into(), "sudo".into())])?,
    }

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["type arkade > /dev/null 2> /dev/null || pkgx curl -sLS https://get.arkade.dev | $SUDO sh"])?
        .stdout()?;
    Ok(stdout)
}
