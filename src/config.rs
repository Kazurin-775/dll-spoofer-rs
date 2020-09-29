pub static LIBNAME_UNTRIMMED: &'static str = include_str!("../libname.cfg");

pub type FunctionName = Option<&'static str>;

include!(concat!(env!("OUT_DIR"), "/func_defs.rs"));
