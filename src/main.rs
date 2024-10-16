use std::str::FromStr;

use topstitch::ModDef;

fn main() {
    let test_sv_path = std::path::PathBuf::from_str("test.sv").unwrap();
    let interface = ModDef::from_verilog_file("my_module", &test_sv_path, false, false);
    let instance = interface.parameterize(&[("BitWidth", 16)], None, None);

    let top = ModDef::new("top");
    let top_autoconnect = &["my_input", "my_output"];
    top.instantiate(&instance, Some("our_instance"), Some(top_autoconnect));

    println!("{}", top.emit(true));
}
