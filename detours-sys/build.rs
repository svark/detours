//! Build detours.
fn build_detours() {
    cc::Build::new()
        .include("deps/detours-src/src/")
        .static_crt(true)
        .flag("/MT")
        .flag("/W4")
        .flag("/WX")
        .flag("/Gy")
        .flag("/Gm-")
        .flag("/Zl")
        .flag("/Od")
        .define("WIN32_LEAN_AND_MEAN", "1")
        .define("_WIN32_WINNT", "0x501")
        .file("deps/detours-src/src/detours.cpp")
        .file("deps/detours-src/src/modules.cpp")
        .file("deps/detours-src/src/disasm.cpp")
        .file("deps/detours-src/src/image.cpp")
        .file("deps/detours-src/src/creatwth.cpp")
        .compile("detours");
}

#[cfg(feature = "buildtime_bindgen")]
fn generate_bindings() {
    use std::{env, fs, path::PathBuf};
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::copy("deps/detours-src/src/detours.h", out_path.join("detours.h")).unwrap();
    //
    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", out_path.to_str().expect("OUTDIR is weird")))
        .clang_arg("-fms-compatibility")
        .clang_arg("-fms-extensions")
        // Detouring APIs
        .allowlist_function("DetourTransactionBegin")
        .allowlist_function("DetourUpdateThread")
        .allowlist_function("DetourAttach")
        .allowlist_function("DetourAttachEx")
        .allowlist_function("DetourAllocateRegionWithingJumpBounds")
        .allowlist_function("DetourDetach")
        .allowlist_function("DetourSetIgnoreTooSmall")
        .allowlist_function("DetourSetRetainRegions")
        .allowlist_function("DetourSetSystemRegionLowerBound")
        .allowlist_function("DetourSetSystemRegionUpperBound")
        .allowlist_function("DetourTransactionAbort")
        .allowlist_function("DetourTransactionCommit")
        .allowlist_function("DetourTransactionCommitEx")
        // Targeting APIs
        .allowlist_function("DetourFindFunction")
        .allowlist_function("DetourCodeFromPointer")
        // Binary and Payload access APIs
        .allowlist_function("DetourEnumerateModules")
        .allowlist_function("DetourGetEntryPoint")
        .allowlist_function("DetourGetModuleSize")
        .allowlist_function("DetourEnumerateExports")
        .allowlist_function("DetourEnumerateImports")
        .allowlist_function("DetourEnumerateImportsEx")
        .allowlist_function("DetourFindPayload")
        .allowlist_function("DetourFindPayloadEx")
        .allowlist_function("DetourFindRemotePayload")
        .allowlist_function("DetourGetContainingModule")
        .allowlist_function("DetourGetSizeOfPayloads")
        // Binary Modifcation APIs
        .allowlist_function("DetourBinaryOpen")
        .allowlist_function("DetourBinaryEnumeratePayloads")
        .allowlist_function("DetourBinaryFindPayload")
        .allowlist_function("DetourBinarySetPayload")
        .allowlist_function("DetourBinaryDeletePayload")
        .allowlist_function("DetourBinaryPurgePayloads")
        .allowlist_function("DetourBinaryEditImports")
        .allowlist_function("DetourBinaryResetImports")
        .allowlist_function("DetourBinaryWrite")
        .allowlist_function("DetourBinaryClose")
        // Injection APIs
        .allowlist_function("DetourCreateProcessWithDllW")
        .allowlist_function("DetourCreateProcessWithDllExW")
        .allowlist_function("DetourCreateProcessWithDllsW")
        .allowlist_function("DetourCreateProcessWithDllA")
        .allowlist_function("DetourCreateProcessWithDllExA")
        .allowlist_function("DetourCreateProcessWithDllsA")
        .allowlist_function("DetourCopyPayloadToProcess")
        .allowlist_function("DetourFinishHelperProcess")
        .allowlist_function("DetourIsHelperProcess")
        .allowlist_function("DetourRestoreAfterWith")
        //
        .header("build/wrapper.h")
        .generate()
        .expect("Unable to generate bindings");
    //
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(not(feature = "buildtime_bindgen"))]
fn generate_bindings() {}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    build_detours();
    generate_bindings();
}
