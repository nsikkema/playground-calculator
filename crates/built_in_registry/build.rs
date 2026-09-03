//! Generates the built-in component registry.

use camino::{Utf8Path, Utf8PathBuf};
use glob::glob;
use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{self, BufWriter, Write};

/// Metadata needed to generate one registry entry.
struct Component {
    /// Stable component identifier.
    id: String,
    /// Component definition version.
    version: u16,
    /// Rust expression referencing the static registration.
    registration_path: String,
}

/// Creates an invalid-data error for malformed component definitions.
fn invalid_data(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

/// Extracts component metadata from a definition source path.
fn component_from_path(manifest_dir: &Utf8Path, path: &Utf8Path) -> io::Result<Component> {
    let component_dir = path
        .parent()
        .and_then(Utf8Path::file_name)
        .ok_or_else(|| invalid_data(format!("invalid component definition path: {path}")))?;
    let (id, version) = component_dir.rsplit_once("_v").ok_or_else(|| {
        invalid_data(format!(
            "component directory must end in _v<version>: {path}"
        ))
    })?;
    let version = version
        .parse::<u16>()
        .map_err(|error| invalid_data(format!("invalid component version in {path}: {error}")))?;
    if version == 0 {
        return Err(invalid_data(format!(
            "component version must be greater than zero: {path}"
        )));
    }

    let relative_path = path
        .strip_prefix(manifest_dir.join("src"))
        .map_err(|error| invalid_data(format!("invalid component path {path}: {error}")))?
        .parent()
        .ok_or_else(|| invalid_data(format!("definition has no component module: {path}")))?
        .as_str()
        .replace('/', "::");

    Ok(Component {
        id: id.to_owned(),
        version,
        registration_path: format!("&crate::{relative_path}::REGISTRATION"),
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = Utf8PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let component_pattern = manifest_dir.join("src/components/**/*_v*/definition.rs");
    let mut components = glob(component_pattern.as_str())?
        .map(|path| {
            let path = Utf8PathBuf::from_path_buf(path?).map_err(|path| {
                invalid_data(format!("component path is not UTF-8: {}", path.display()))
            })?;
            component_from_path(&manifest_dir, &path)
        })
        .collect::<Result<Vec<_>, io::Error>>()?;
    components
        .sort_unstable_by(|left, right| (&left.id, left.version).cmp(&(&right.id, right.version)));

    let mut latest = BTreeMap::<String, (u16, String)>::new();
    let mut versions = BTreeMap::<String, Vec<u16>>::new();
    let mut component_map = phf_codegen::Map::<(&str, u16)>::new();
    for component in &components {
        component_map.entry(
            (component.id.as_str(), component.version),
            component.registration_path.clone(),
        );
        latest.insert(
            component.id.clone(),
            (component.version, component.registration_path.clone()),
        );
        versions
            .entry(component.id.clone())
            .or_default()
            .push(component.version);
    }

    let mut latest_map = phf_codegen::Map::<&str>::new();
    for (id, (_version, registration_path)) in &latest {
        latest_map.entry(id, registration_path);
    }

    let mut versions_map = phf_codegen::Map::<&str>::new();
    for (id, component_versions) in &versions {
        let values = component_versions
            .iter()
            .map(|version| format!("{version}u16"))
            .collect::<Vec<_>>()
            .join(", ");
        versions_map.entry(id, format!("&[{values}]"));
    }

    let output_path = Utf8PathBuf::from(env::var("OUT_DIR")?).join("built_in_registry.rs");
    let mut output = BufWriter::new(File::create(output_path)?);
    writeln!(
        output,
        "/// All built-in registrations, keyed by `(id, version)`.\n\
         #[allow(clippy::unreadable_literal)]\n\
        pub static COMPONENTS: phf::Map<(&'static str, u16), &'static crate::BuiltInComponentRegistration> =\n{};\n\
        /// Latest built-in registration for each component id.\n\
         #[allow(clippy::unreadable_literal)]\n\
        pub static LATEST: phf::Map<&'static str, &'static crate::BuiltInComponentRegistration> =\n{};\n\
         /// Available versions for each component id, in ascending order.\n\
         #[allow(clippy::unreadable_literal)]\n\
         pub static VERSIONS: phf::Map<&'static str, &'static [u16]> =\n{};",
        component_map.build(),
        latest_map.build(),
        versions_map.build(),
    )?;

    println!("cargo::rerun-if-changed=src/components");
    Ok(())
}
