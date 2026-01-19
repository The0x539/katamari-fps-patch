fn main() -> Result<(), String> {
    if cfg!(windows) {
        let mut vcvars = vcvars::Vcvars::new();
        let path = vcvars.get_cached("PATH").unwrap();
        unsafe {
            std::env::set_var("PATH", path.as_ref());
        }
    }

    nasm_rs::compile_library(
        "native_replacements.lib",
        &["src/replacements/abilities.asm"],
    )
}
