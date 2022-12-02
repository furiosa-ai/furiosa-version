use std::ffi::CStr;
use std::fmt::Write;
use std::os::raw::c_char;

use clap::ArgMatches;
use eyre::Context;
use libloading::{Library, Symbol};

#[derive(Debug)]
struct VersionInfo {
    version: String,
    hash: String,
    build_time: String,
}

fn npu_tools_lib(libname: &str) -> eyre::Result<VersionInfo> {
    unsafe {
        let lib = Library::new(format!("{}.so", libname))
            .with_context(|| format!("{}.so not found", libname))?;
        let version: Symbol<'_, extern "C" fn() -> *const c_char> = lib.get(b"version").unwrap();
        let git_hash: Symbol<'_, extern "C" fn() -> *const c_char> =
            lib.get(b"git_short_hash").unwrap();
        let build_timestamp: Symbol<'_, extern "C" fn() -> *const c_char> =
            lib.get(b"build_timestamp").unwrap();

        Ok(VersionInfo {
            version: convert_cstr(version()),
            hash: convert_cstr(git_hash()),
            build_time: convert_cstr(build_timestamp()),
        })
    }
}

fn libcompiler() -> eyre::Result<VersionInfo> {
    unsafe {
        let lib = Library::new("libfuriosa_compiler.so")
            .with_context(|| "libfuriosa_compiler.so not found")?;
        let version: Symbol<'_, extern "C" fn() -> *const c_char> = lib.get(b"fc_version").unwrap();
        let git_hash: Symbol<'_, extern "C" fn() -> *const c_char> =
            lib.get(b"fc_revision").unwrap();
        let build_timestamp: Symbol<'_, extern "C" fn() -> *const c_char> =
            lib.get(b"fc_buildtime").unwrap();

        Ok(VersionInfo {
            version: convert_cstr(version()),
            hash: convert_cstr(git_hash()),
            build_time: convert_cstr(build_timestamp()),
        })
    }
}

fn convert_cstr(s: *const c_char) -> String {
    unsafe {
        CStr::from_ptr(s)
            .to_str()
            .expect("Invalid UTF-8 encoding")
            .to_string()
    }
}

fn print_version(matches: &ArgMatches, vinfo: VersionInfo) -> eyre::Result<()> {
    let mut sb = String::new();

    // If there's no flag, print out all of version strings.
    let no_flag = !(matches.get_flag("version")
        || matches.get_flag("hash")
        || matches.get_flag("build-time"));

    let mut precedent = false;

    if no_flag || matches.get_flag("version") {
        sb.write_str(&vinfo.version)?;
        precedent = true;
    }
    if no_flag || matches.get_flag("hash") {
        if precedent {
            sb.write_char(' ')?;
        }
        sb.write_str(&vinfo.hash)?;
        precedent = true;
    }
    if no_flag || matches.get_flag("build-time") {
        if precedent {
            sb.write_char(' ')?;
        }
        sb.write_str(&vinfo.build_time)?;
    }

    println!("{}", sb);
    Ok(())
}

fn main() -> eyre::Result<()> {
    let cmd = clap::Command::new("furiosa-version")
        .bin_name("furiosa-version")
        .args(vec![
            clap::arg!([name] "an library to get (available names: libhal, libruntime, libcompiler)")
                .value_parser(clap::value_parser!(String))
                .index(1)
                .required(true),
            clap::arg!(--version "Get a version string").default_value("false"),
            clap::arg!(--hash "Get a git hash string").default_value("false"),
            clap::arg!(--"build-time" "Get a build time").default_value("false"),
        ])
        .arg_required_else_help(true)
        .after_help(r#"Examples:

  # Print a git hash of libcompiler
  furiosa-version libcompiler --hash

  # Print a version and build time of libhal
  furiosa-version libhal --version --build-time
        "#);

    let matches = cmd.get_matches();
    let name = matches.get_one::<String>("name").unwrap();

    match name.as_ref() {
        "libhal" => print_version(&matches, npu_tools_lib("libfuriosa_hal")?),
        "libruntime" => print_version(&matches, npu_tools_lib("libfuriosa_runtime")?),
        "libcompiler" => print_version(&matches, libcompiler()?),
        _ => panic!("unknown {}", name),
    }
}
