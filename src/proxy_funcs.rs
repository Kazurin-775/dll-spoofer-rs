use crate::config;

macro_rules! proxy_func {
    ($func_name:ident, $index:literal) => {
        #[no_mangle]
        #[naked]
        #[allow(non_snake_case)]
        pub unsafe fn $func_name() {
            asm!("jmp rax", in("rax") config::FUNCTIONS[$index]);
        }
    };
}

include!(concat!(env!("OUT_DIR"), "/proxy_funcs.rs"));
