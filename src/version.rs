pub fn get_version_from_cargo_toml() -> Result<String, toml::de::Error> {
    const CARGO_TOML: &str = include_str!("../Cargo.toml");

    let cargo_toml: toml::Value = toml::from_str(CARGO_TOML)?;
    let version = cargo_toml
        .get("package")
        .and_then(|package| package.get("version"))
        .and_then(|version| version.as_str())
        .unwrap_or("Unknown");

    Ok(version.to_string())
}