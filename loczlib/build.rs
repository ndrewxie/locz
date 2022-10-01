use fs::{DirEntry, File};
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

const ERR_MSG: &'static str = "Error bundling assets";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets");

    let base_dir = env::var_os("OUT_DIR").unwrap();
    let out_dir = Path::new(&base_dir).join("assets");
    let input_dir = Path::new("./assets");
    let mut output_module = String::from("");

    if out_dir.is_dir() {
        fs::remove_dir_all(&out_dir).expect(ERR_MSG);
    }
    fs::create_dir_all(&out_dir).expect(ERR_MSG);

    for entry in fs::read_dir(input_dir).expect(ERR_MSG) {
        if let Ok(asset) = entry {
            add_asset(asset, &out_dir, &mut output_module);
            output_module.push('\n');
        }
    }

    let mut module_file = File::create(out_dir.join("assets.rs")).expect(ERR_MSG);
    module_file.write(output_module.as_bytes()).expect(ERR_MSG);
}

fn add_asset(entry: DirEntry, out_dir: &Path, output_module: &mut String) {
    let asset = entry.path();
    if !asset.is_file() {
        return;
    }
    let asset_file = File::open(&asset).unwrap();
    let decoder = png::Decoder::new(asset_file);
    let mut reader = decoder.read_info().expect(ERR_MSG);
    let mut buffer = vec![0; reader.output_buffer_size()];
    let frame_info = reader.next_frame(&mut buffer).expect(ERR_MSG);
    let bytes = &buffer[..frame_info.buffer_size()];

    let mut asset_ident = asset
        .file_stem()
        .expect(ERR_MSG)
        .to_os_string()
        .into_string()
        .expect(ERR_MSG);
    asset_ident.make_ascii_uppercase();

    let mut asset_name = asset_ident.clone();
    asset_name.push_str(".rgba");

    let mut out_file = File::create(out_dir.join(&asset_name)).expect(ERR_MSG);
    out_file.write_all(bytes).expect(ERR_MSG);

    output_module.push_str(&format!(
        "pub const {}: &'static [u8; {}] = include_bytes!(\"{}\");\n",
        &asset_ident,
        &bytes.len().to_string(),
        format!("{}{}", "./", &asset_name)
    ));
    output_module.push_str(&format!(
        "pub const DIMS_{}: (usize, usize) = ({}, {});\n",
        &asset_ident, frame_info.width, frame_info.height,
    ));
}
