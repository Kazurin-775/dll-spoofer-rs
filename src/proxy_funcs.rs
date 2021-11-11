use crate::config;

macro_rules! proxy_func {
    ($func_name:ident, $index:literal) => {
        #[no_mangle]
        #[naked]
        #[allow(non_snake_case)]
        pub unsafe extern fn $func_name() {
            // Since Rust only allows using `sym` and `const` in a naked
            // function now, we have to manually compute the address of
            // the function pointer, using the address of `config::FUNCTIONS`
            // and the array index.
            //
            // When this line is fed into the assembler, it becomes sth like
            // `jmp [rip + some_mangled_name + some_number]`.
            // This provides sufficient information for the assembler to do
            // the math for us.
            //
            // The old-fashioned way of using `&config::FUNCTIONS[$index]`
            // does not work anymore.
            asm!(
                "jmp [rip + {base} + {offset}]",
                base = sym config::FUNCTIONS,
                offset = const (std::mem::size_of::<*const ()>() * $index),
                options(noreturn),
            );
        }
    };
}

include!(concat!(env!("OUT_DIR"), "/proxy_funcs.rs"));
