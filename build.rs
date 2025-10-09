use std::process::Command;

fn main() {
    // Kompiliere die GSettings-Schemas bei jedem Build
    let schema_dir = "data";

    let status = Command::new("glib-compile-schemas")
        .arg(schema_dir)
        .status()
        .expect("Fehler beim Ausführen von glib-compile-schemas");

    if !status.success() {
        panic!("glib-compile-schemas fehlgeschlagen");
    }

    // Sag Cargo, dass es neu bauen soll, wenn sich das Schema ändert
    println!("cargo:rerun-if-changed={}/dem.hauke.Impfungen.gschema.xml", schema_dir);

    // Übergib das Schema-Verzeichnis an das Programm
    println!("cargo:rustc-env=GSETTINGS_SCHEMA_DIR={}", schema_dir);
}