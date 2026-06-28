use anyhow::{bail, Context, Result};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;

pub fn convert_lines(binary: &Path, config: &Path, input: &[String]) -> Result<Vec<String>> {
    if input.is_empty() {
        return Ok(Vec::new());
    }

    let mut child = Command::new(binary)
        .arg("-c")
        .arg(config)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("spawn OpenCC CLI {}", binary.display()))?;

    let expected_len = input.len();
    let mut stdin = child.stdin.take().context("open OpenCC stdin")?;
    let input = input.to_owned();
    let writer = thread::spawn(move || -> Result<()> {
        for line in input {
            stdin
                .write_all(line.as_bytes())
                .context("write phrase to OpenCC stdin")?;
            stdin.write_all(b"\n").context("write OpenCC newline")?;
        }
        Ok(())
    });

    let output = child.wait_with_output().context("wait for OpenCC CLI")?;
    writer
        .join()
        .map_err(|_| anyhow::anyhow!("OpenCC stdin writer thread panicked"))??;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!(
            "OpenCC CLI failed with {}: {}",
            output.status,
            stderr.trim()
        );
    }

    let text = String::from_utf8(output.stdout).context("decode OpenCC stdout")?;
    let converted = text.lines().map(str::to_string).collect::<Vec<_>>();
    if converted.len() != expected_len {
        bail!(
            "OpenCC returned {} lines for {} input lines",
            converted.len(),
            expected_len
        );
    }
    Ok(converted)
}
