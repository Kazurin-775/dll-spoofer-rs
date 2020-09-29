use std::ffi::OsString;
use std::fs::File;
use std::io::Write;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;

use pelite::{util::CStr, FileMap, PeFile};
use winapi::um::sysinfoapi::GetSystemDirectoryW;

fn get_libpath() -> PathBuf {
    let len = unsafe { GetSystemDirectoryW(std::ptr::null_mut(), 0) };
    let mut buf = Vec::<u16>::new();
    buf.resize(len as usize, 0);
    unsafe {
        assert_eq!(GetSystemDirectoryW(buf.as_mut_ptr(), len), len - 1);
    }
    let mut path_buf = PathBuf::from(OsString::from_wide(&buf[0..len as usize - 1]));
    path_buf.push(format!("{}.dll", include_str!("libname.cfg").trim()));
    dbg!(&path_buf);
    path_buf
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let libpath = get_libpath();
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());

    // read exports from the DLL file

    let mapping = FileMap::open(&libpath)?;
    let pe_file = PeFile::from_bytes(&mapping)?;
    let exports = pe_file.exports()?.by()?;

    let mut export_names = Vec::<Option<&CStr>>::new();
    export_names.resize(exports.functions().len(), None);

    for export in exports.iter_name_indices() {
        export_names[export.1] = Some(export.0?);
    }

    let ordinal_base = pe_file.exports()?.ordinal_base() as usize;

    // generate func_defs.rs

    let mut func_defs = File::create(out_dir.join("func_defs.rs"))?;
    writeln!(
        &mut func_defs,
        "pub static ORDINAL_BASE: u16 = {};",
        ordinal_base
    )?;
    writeln!(
        &mut func_defs,
        "pub static mut FUNCTIONS: [*const (); {}] = [std::ptr::null(); {}];",
        export_names.len(),
        export_names.len()
    )?;
    writeln!(
        &mut func_defs,
        "pub static FUNCTION_NAMES: [FunctionName; {}] = [",
        export_names.len()
    )?;
    for export_name in export_names.iter() {
        if let Some(name) = export_name {
            assert!(!name.contains(&('"' as u8)) && !name.contains(&('\\' as u8)));
            writeln!(&mut func_defs, "Some(\"{}\\0\"),", name)?;
        } else {
            writeln!(&mut func_defs, "None,")?;
        }
    }
    writeln!(&mut func_defs, "];")?;
    func_defs.flush()?;

    // generate proxy_funcs.rs and exports.def

    let mut proxy_funcs = File::create(out_dir.join("proxy_funcs.rs"))?;
    let mut exports_def = File::create(out_dir.join("exports.def"))?;
    writeln!(&mut exports_def, "LIBRARY dll-spoofer-rs\nEXPORTS")?;

    for (i, export_name) in export_names.iter().enumerate() {
        if let Some(name) = export_name {
            writeln!(&mut proxy_funcs, "proxy_func!({}, {});", name, i)?;
            writeln!(&mut exports_def, "{} @{}", name, i + ordinal_base)?;
        } else {
            writeln!(&mut proxy_funcs, "proxy_func!(unkExport{}, {});", i, i)?;
            writeln!(&mut exports_def, "unkExport{} @{}", i, i + ordinal_base)?;
        }
    }
    proxy_funcs.flush()?;
    exports_def.flush()?;

    // write compiler flags

    println!(
        "cargo:rustc-cdylib-link-arg=/def:{}",
        out_dir.join("exports.def").to_str().unwrap()
    );

    Ok(())
}
