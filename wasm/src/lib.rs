#[path = "../../src/util.rs"]
mod util;

#[path = "../../src/validate.rs"]
mod validate;

#[no_mangle]
pub extern "C" fn validate(value: u64, len: u32) -> u32 {
    let width = len as usize;
    if !(1..=14).contains(&width) {
        return 0;
    }
    let text = format!("{value:0width$}");
    if text.len() != width {
        return 0;
    }
    match width {
        11 if validate::cpf(&text) => 1,
        14 if validate::cnpj(&text) => 2,
        _ => 0,
    }
}
