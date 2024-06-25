use std::{collections::HashSet, env, path::PathBuf};

macro_rules! add_if_feature {
    ($s:tt) => {
        #[cfg(feature = $s)]
        $s
    };
}

const ENABLED_LIBRARIES: &[&str] = &[
    add_if_feature!("mongoose"),
    add_if_feature!("amd"),
    add_if_feature!("btf"),
    add_if_feature!("camd"),
    add_if_feature!("ccolamd"),
    add_if_feature!("colamd"),
    add_if_feature!("cholmod"),
    add_if_feature!("cxsparse"),
    add_if_feature!("ldl"),
    add_if_feature!("klu"),
    add_if_feature!("umfpack"),
    add_if_feature!("paru"),
    add_if_feature!("rbio"),
    add_if_feature!("spqr"),
    add_if_feature!("spex"),
    add_if_feature!("graphblas"),
    add_if_feature!("lagraph"),
];

// Ignore some macros (based on https://github.com/rust-lang/rust-bindgen/issues/687#issuecomment-1312298570)
#[derive(Debug)]
struct IgnoreMacros(HashSet<&'static str>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        use bindgen::callbacks::MacroParsingBehavior;
        if self.0.contains(name) {
            MacroParsingBehavior::Ignore
        } else {
            MacroParsingBehavior::Default
        }
    }
}

impl IgnoreMacros {
    const IGNORE_CONSTANTS: [&'static str; 19] = [
        "FE_DIVBYZERO",
        "FE_DOWNWARD",
        "FE_INEXACT",
        "FE_INVALID",
        "FE_OVERFLOW",
        "FE_TONEAREST",
        "FE_TOWARDZERO",
        "FE_UNDERFLOW",
        "FE_UPWARD",
        "FP_INFINITE",
        "FP_INT_DOWNWARD",
        "FP_INT_TONEAREST",
        "FP_INT_TONEARESTFROMZERO",
        "FP_INT_TOWARDZERO",
        "FP_INT_UPWARD",
        "FP_NAN",
        "FP_NORMAL",
        "FP_SUBNORMAL",
        "FP_ZERO",
    ];

    fn new() -> Self {
        Self(Self::IGNORE_CONSTANTS.iter().copied().collect())
    }
}

#[derive(Debug)]
struct Library {
    inc: Option<String>,
    lib: Option<String>,
    is_static: bool,
}

impl Library {
    fn new() -> Self {
        Self {
            inc: env::var("SUITESPARSE_INCLUDE_DIR").ok(),
            lib: env::var("SUITESPARSE_LIBRARY_DIR").ok(),
            is_static: true,
        }
    }
    fn is_some(&self) -> bool {
        self.inc.is_some() && self.lib.is_some()
    }
}

fn build_vendor() -> Result<Library, String> {
    macro_rules! feature {
        ($s:tt) => {
            if cfg!(feature = $s) {
                "ON"
            } else {
                "OFF"
            }
        };
    }

    let (mut static_libraries, shared_libraries) = if cfg!(feature = "static_libraries") {
        ("ON", "OFF")
    } else {
        ("OFF", "ON")
    };

    if static_libraries == "OFF" && shared_libraries == "OFF" {
        println!("cargo:warning=Both static_libraries and shared_libraries features are disabled. Defaulting to static_libraries.");
        static_libraries = "ON";
    }

    let mut build_libraries = vec!["suitesparse_config"];
    for &lib in ENABLED_LIBRARIES {
        build_libraries.push(lib);
    }

    let mut config = cmake::Config::new("vendor");
    config
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        .define("CMAKE_INSTALL_INCLUDEDIR", "include")
        .define("BUILD_STATIC_LIBS", static_libraries)
        .define("BUILD_SHARED_LIBS", shared_libraries)
        .define("SUITESPARSE_ENABLE_PROJECTS", build_libraries.join(";"))
        .define("SUITESPARSE_DEMOS", "OFF")
        .define("SUITESPARSE_CONFIG_USE_OPENMP", feature!("openmp"));

    let dst = config.build();
    let dst_disp = dst.display();
    let lib_loc = Some(format!("{}/lib", dst_disp));
    let inc_dir = Some(format!("{}/include", dst_disp));
    Ok(Library {
        inc: inc_dir,
        lib: lib_loc,
        is_static: static_libraries == "ON",
    })
}

fn generate_bindings(suitesparse: &Library) -> Result<(), String> {
    let mut lib_args = vec![];
    for &lib in ENABLED_LIBRARIES {
        lib_args.push(format!("-DUSE_{}", lib.to_uppercase()));
    }

    let builder = bindgen::Builder::default().header("wrapper.h");
    let bindings = builder
        .clang_arg(format!("-I{}", suitesparse.inc.as_ref().unwrap()))
        .clang_args(lib_args)
        .parse_callbacks(Box::new(IgnoreMacros::new()))
        .generate()
        .map_err(|e| e.to_string())?;

    let bindings_rs = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");

    bindings
        .write_to_file(bindings_rs)
        .expect("Couldn't write file bindings.rs!");
    Ok(())
}

fn main() -> Result<(), String> {
    // try to find suitesparse
    let mut suitesparse = Library::new();

    // if we can't find suitesparse or the build_vendor feature is enabled, we build the vendor suitesparse library
    if cfg!(feature = "build_vendor") {
        suitesparse = build_vendor()?;
    } else if !suitesparse.is_some() {
        println!("cargo:warning=SUITEPARSE_INCLUDE_DIR and SUITESPARSE_LIBRARY_DIR are not set. Building vendor suitesparse library.");
        suitesparse = build_vendor()?;
    } else {
        assert!(suitesparse.inc.is_some(), "Cannot find suitesparse include and library directories and build_vendor feature is not enabled.");
    }

    // generate bindings to found or build suitesparse
    generate_bindings(&suitesparse)?;

    // let Cargo know about the library files
    println!(
        "cargo:rustc-link-search=native={}",
        suitesparse.lib.as_ref().unwrap()
    );

    let mut lib_names = vec!["suitesparseconfig"];
    for &lib in ENABLED_LIBRARIES {
        lib_names.push(lib);
    }

    let library_type = if suitesparse.is_static {
        "static"
    } else {
        "dylib"
    };

    for lib_name in &lib_names {
        println!("cargo:rustc-link-lib={}={}", library_type, lib_name);
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");

    Ok(())
}
