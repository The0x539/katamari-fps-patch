fn main() -> Result<(), String> {
    nasm_rs::compile_library(
        "native_replacements.lib",
        &["src/replacements/abilities.asm"],
    )
}
