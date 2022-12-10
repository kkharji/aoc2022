use phf_codegen::Map;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn read_input_dir(dir: &str, folder: &str, mut f: impl FnMut(String, String)) {
    let path = format!("{dir}/input/{folder}");
    println!("cargo:rerun-if-changed={path}");

    for entry in std::fs::read_dir(path).expect("reading directory") {
        let Ok(entry) = entry  else { continue };
        let path = entry.path();
        let Some(name) = path.file_stem().and_then(|v| v.to_str()) else { continue; };
        let content = std::fs::read_to_string(&path).expect("Read data file");
        f(name.to_string(), format!("r#\"{content}\"#"))
    }
}

const TYPE: &str = "phf::Map<&'static str, &'static str>";

fn main() {
    let src_path = std::env!("CARGO_MANIFEST_DIR");
    let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = std::io::BufWriter::new(File::create(&out_path).unwrap());

    let mut examples = Map::new();

    read_input_dir(&src_path, "examples", |name, content| {
        examples.entry(name, content.as_str());
    });
    let examples = examples.build();

    write!(&mut file, "static AOC_EXAMPLES: {TYPE} = {examples};",).unwrap();

    let mut data = Map::new();
    read_input_dir(&src_path, "data", |name, content| {
        data.entry(name, content.as_str());
    });
    let data = data.build();

    write!(&mut file, "static AOC_DATA: {TYPE} = {data};",).unwrap();
}
