use std::error::Error;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fmt, fs};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn build() -> Result<()> {
    let root_dir = root_dir()?;
    // TODO: get this from a config file
    let crate_name = root_dir
        .file_name()
        .ok_or::<Box<dyn Error>>("Freight run in directory without a name".into())?
        .to_str()
        .unwrap();
    let lib_rs = root_dir.join("src").join("lib.rs");
    let main_rs = root_dir.join("src").join("main.rs");
    let target = root_dir.join("target");
    let target_debug = target.join("debug");
    fs::create_dir_all(&target_debug)?;

    let compile_lib = || -> Result<()> {
        println!("Compiling lib.rs");
        Rustc::builder()
            .edition(Edition::E2021)
            .crate_type(CrateType::Lib)
            .crate_name(crate_name)
            .out_dir(&target_debug)
            .lib_dir(&target_debug)
            .build()
            .run(&lib_rs)?;
        println!("Compiling lib.rs -- DONE");
        Ok(())
    };

    let compile_bin = |externs: Vec<&str>| -> Result<()> {
        println!("Compiling main.rs");
        let mut builder = Rustc::builder()
            .edition(Edition::E2021)
            .crate_type(CrateType::Bin)
            .crate_name(crate_name)
            .out_dir(&target_debug)
            .lib_dir(&target_debug);
        for ex in externs {
            builder = builder.externs(ex);
        }
        builder.build().run(&main_rs)?;
        println!("Compiling main.rs -- DONE");
        Ok(())
    };

    match (lib_rs.exists(), main_rs.exists()) {
        (true, true) => {
            compile_lib()?;
            compile_bin(vec![crate_name])?;
        }
        (true, false) => {
            compile_lib()?;
        }
        (false, true) => {
            compile_bin(vec![])?;
        }
        (false, false) => return Err("There is nothing to compile".into()),
    }

    Ok(())
}

pub struct Rustc {
    edition: Edition,
    crate_type: CrateType,
    crate_name: String,
    out_dir: PathBuf,
    lib_dir: PathBuf,
    cfg: Vec<String>,
    externs: Vec<String>,
}

pub enum Edition {
    E2015,
    E2018,
    E2021,
}

pub enum CrateType {
    Bin,
    Lib,
    RLib,
    DyLib,
    CDyLib,
    StaticLib,
    ProcMacro,
}

#[derive(Default)]
pub struct RustcBuilder {
    edition: Option<Edition>,
    crate_type: Option<CrateType>,
    crate_name: Option<String>,
    out_dir: Option<PathBuf>,
    lib_dir: Option<PathBuf>,
    cfg: Vec<String>,
    externs: Vec<String>,
}

impl fmt::Display for Edition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let edition = match self {
            Self::E2015 => "2015",
            Self::E2018 => "2018",
            Self::E2021 => "2021",
        };
        write!(f, "{edition}")
    }
}

impl fmt::Display for CrateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let crate_type = match self {
            Self::Bin => "bin",
            Self::Lib => "lib",
            Self::RLib => "rlib",
            Self::DyLib => "dylib",
            Self::CDyLib => "cdylib",
            Self::StaticLib => "staticlib",
            Self::ProcMacro => "proc-macro",
        };
        write!(f, "{crate_type}")
    }
}

impl Rustc {
    pub fn builder() -> RustcBuilder {
        Default::default()
    }

    pub fn run<S: AsRef<OsStr>>(self, path: &S) -> Result<()> {
        Command::new("rustc")
            .arg(path)
            .arg("--edition")
            .arg(self.edition.to_string())
            .arg("--crate-type")
            .arg(self.crate_type.to_string())
            .arg("--crate-name")
            .arg(self.crate_name.to_string())
            .arg("--out-dir")
            .arg(self.out_dir)
            .arg("-L")
            .arg(self.lib_dir)
            .args(self.externs.iter().map(|x| format!("--extern={x}")))
            .args(self.cfg.iter().map(|x| format!("--cfg={x}")))
            .spawn()?
            .wait()?;
        Ok(())
    }
}

impl RustcBuilder {
    pub fn edition(mut self, edition: Edition) -> Self {
        self.edition = Some(edition);
        self
    }
    pub fn out_dir(mut self, out_dir: impl Into<PathBuf>) -> Self {
        self.out_dir = Some(out_dir.into());
        self
    }
    pub fn lib_dir(mut self, lib_dir: impl Into<PathBuf>) -> Self {
        self.lib_dir = Some(lib_dir.into());
        self
    }
    pub fn crate_name(mut self, crate_name: impl Into<String>) -> Self {
        self.crate_name = Some(crate_name.into());
        self
    }
    pub fn crate_type(mut self, crate_type: CrateType) -> Self {
        self.crate_type = Some(crate_type);
        self
    }
    pub fn cfg(mut self, cfg: impl Into<String>) -> Self {
        self.cfg.push(cfg.into());
        self
    }
    pub fn externs(mut self, r#extern: impl Into<String>) -> Self {
        self.externs.push(r#extern.into());
        self
    }
    pub fn build(self) -> Rustc {
        Rustc {
            edition: self.edition.unwrap_or(Edition::E2015),
            crate_type: self.crate_type.unwrap_or(CrateType::Bin),
            crate_name: self.crate_name.expect("Crate name given"),
            out_dir: self.out_dir.expect("Out dir given"),
            lib_dir: self.lib_dir.expect("Lib dir given"),
            cfg: self.cfg,
            externs: self.externs,
        }
    }
}

fn root_dir() -> Result<PathBuf> {
    let current_dir = env::current_dir()?;
    for ancestor in current_dir.ancestors() {
        if ancestor.join(".git").exists() {
            return Ok(ancestor.into());
        }
    }
    Err("No root dir".into())
}
