use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    let openfang = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join("openfang")));

    let Some(openfang_path) = openfang else {
        eprintln!("Failed to resolve openfang binary path.");
        return ExitCode::from(1);
    };

    let status = Command::new(&openfang_path)
        .args(std::env::args().skip(1))
        .status();

    match status {
        Ok(status) => {
            let code = status
                .code()
                .and_then(|c| u8::try_from(c).ok())
                .unwrap_or(1);
            ExitCode::from(code)
        },
        Err(err) => {
            eprintln!(
                "Unable to launch '{}' from ochi alias: {err}",
                openfang_path.display()
            );
            ExitCode::from(1)
        }
    }
}
