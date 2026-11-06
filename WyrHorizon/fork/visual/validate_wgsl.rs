#[path="../../horizon/crates/horizon-client-render/src/overlay.rs"]
#[allow(dead_code)] mod overlay;
fn main() {
    for path in std::env::args().skip(1) {
        let mut source=std::fs::read_to_string(&path).unwrap();
        if path.ends_with("entity.wgsl") {source=overlay::wgsl_constants()+&source;}
        if path.ends_with("visbuffer.wgsl") {
            let base=std::path::Path::new(&path).with_file_name("terrain.wgsl");
            source=std::fs::read_to_string(base).unwrap()+&source;
        }
        let module=naga::front::wgsl::parse_str(&source).unwrap_or_else(|e|panic!("{}: {}",path,e.emit_to_string(&source)));
        naga::valid::Validator::new(naga::valid::ValidationFlags::all(),naga::valid::Capabilities::all())
            .validate(&module).unwrap_or_else(|e|panic!("{}: {:?}",path,e));
        println!("PASS WGSL {} ({} entry points)",path,module.entry_points.len());
    }
}
