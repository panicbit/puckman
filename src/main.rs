use std::io::{Write, BufWriter};

use alpm::{Alpm, Package, PackageFrom, PackageReason};
use anyhow::*;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use serde::Serialize;

fn main() -> Result<()> {
    let alpm = Alpm::new2("/", "/var/lib/pacman")?;
    let db = alpm.localdb();
    let pkgs = db.pkgs();

    let stdout = std::io::stdout().lock();
    let mut stdout = BufWriter::new(stdout);

    for pkg in pkgs {
        let package_info = PackageInfo::from(pkg);

        serde_json::to_writer(&mut stdout, &package_info)?;
        writeln!(stdout)?;
    }

    Ok(())
}

#[derive(Serialize)]
struct PackageInfo<'a> {
    name: &'a str,
    filename: &'a str,
    base: Option<&'a str>,
    version: &'a str,
    origin: Origin,
    description: Option<&'a str>,
    url: Option<&'a str>,
    build_date: DateTime<Local>,
    install_date: Option<DateTime<Local>>,
    packager: Option<&'a str>,
    md5sum: Option<&'a str>,
    sha256sum: Option<&'a str>,
    arch: Option<&'a str>,
    package_size: i64,
    installed_size: i64,
    install_reason: InstallReason,
    licenses: Vec<&'a str>,
    groups: Vec<&'a str>,
    dependencies: Vec<&'a str>,
    optional_dependencies: Vec<&'a str>,
    check_dependencies: Vec<&'a str>,
    make_dependencies: Vec<&'a str>,
    conflicts: Vec<&'a str>,
    provides: Vec<&'a str>,
    replaces: Vec<&'a str>,
    required_by: Vec<String>,
    optional_for: Vec<String>,
    signature: Option<&'a str>,
    has_scriptlet: bool,
}

impl<'a> From<Package<'a>> for PackageInfo<'a> {
    fn from(pkg: Package<'a>) -> Self {
        Self {
            name: pkg.name(),
            filename: pkg.filename(),
            base: pkg.base(),
            version: pkg.version().as_str(),
            origin: Origin::from(pkg.origin()),
            description: pkg.desc(),
            url: pkg.url(),
            build_date: timestamp_to_datetime(pkg.build_date()),
            install_date: pkg.install_date().map(timestamp_to_datetime),
            packager: pkg.packager(),
            md5sum: pkg.md5sum(),
            sha256sum: pkg.sha256sum(),
            arch: pkg.arch(),
            package_size: pkg.size(),
            installed_size: pkg.isize(),
            install_reason: InstallReason::from(pkg.reason()),
            licenses: pkg.licenses().iter().collect(),
            groups: pkg.groups().iter().collect(),
            dependencies: pkg.depends().iter().map(|dep| dep.name()).collect(),
            optional_dependencies: pkg.optdepends().iter().map(|dep| dep.name()).collect(),
            check_dependencies: pkg.checkdepends().iter().map(|dep| dep.name()).collect(),
            make_dependencies: pkg.makedepends().iter().map(|dep| dep.name()).collect(),
            conflicts: pkg.conflicts().iter().map(|dep| dep.name()).collect(),
            provides: pkg.provides().iter().map(|dep| dep.name()).collect(),
            replaces: pkg.replaces().iter().map(|dep| dep.name()).collect(),
            required_by: pkg.required_by().into_iter().collect(),
            optional_for: pkg.optional_for().into_iter().collect(),
            signature: pkg.base64_sig(),
            has_scriptlet: pkg.has_scriptlet(),
        }
    }
}

fn timestamp_to_datetime(timestamp: i64) -> DateTime<Local> {
    let datetime = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap_or_default();

    Local.from_utc_datetime(&datetime)
}

#[derive(Serialize)]
enum Origin {
    File,
    LocalDb,
    SyncDb,
}

impl From<PackageFrom> for Origin {
    fn from(package_from: PackageFrom) -> Self {
        match package_from {
            PackageFrom::File => Self::File,
            PackageFrom::LocalDb => Self::LocalDb,
            PackageFrom::SyncDb => Self::SyncDb,
        }
    }
}

#[derive(Serialize)]
enum InstallReason {
    Explicit,
    Dependency,
}

impl From<PackageReason> for InstallReason {
    fn from(package_reason: PackageReason) -> Self {
        match package_reason {
            PackageReason::Explicit => Self::Explicit,
            PackageReason::Depend => Self::Dependency,
        }
    }
}