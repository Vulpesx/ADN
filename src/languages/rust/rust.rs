use crate::utilities::read;
use std::path::PathBuf;
use std::process::Command;

pub fn add_rust(helix_lang_conf: &PathBuf) {
    let output = Command::new("rustup")
        .arg("component")
        .arg("add")
        .arg("rust-analyzer")
        .output()
        .expect("\n-Falló al ejecutar `rustup component add rust-analyzer`\n-Verifique que Rust este instalado\n-Verifique que rustup este instalado");

    if output.status.success() {
        println!("Instalación de `rustup component add rust-analyzer` completa");

        let config_to_add = r#"
[[language]]
name = "rust"

[language-server.rust-analyzer.config.check]
command = "clippy"
scope = "source.rust"
injection-regex = "rust"
file-types = ["rs"]
roots = ["Cargo.toml", "Cargo.lock"]
auto-format = true
comment-token = "//"
language-server = { command = "rust-analyzer" }
indent = { tab-width = 4, unit = "    " }
"#;
        if let Err(err) = read(helix_lang_conf, config_to_add) {
            eprintln!("Error al leer el archivo: {err}");
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error al ejecutar el comando:\n{stderr}");
    }
}
