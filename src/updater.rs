use anyhow::Result;
use std::io::Write;

const STABLE_URL: &str = "https://github.com/HasSak-47/loteria_cli/releases/download/stable/loteria_cli.exe";

pub fn update() -> Result<()>{
    use std::process::Stdio;

    let mut cmd = std::process::Command::new("PowerShell.exe");
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    let mut spawn = cmd.spawn()?;
    let stdin = spawn.stdin.as_mut().unwrap();
    stdin.write(&format!("wget {} -OutFile loteria_cli.exe\n", STABLE_URL).as_bytes())?;
    spawn.wait()?;

    Ok(())
}

