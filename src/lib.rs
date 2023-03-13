#![cfg_attr(
    not(feature = "std"),
    no_std,
    feature(lang_items),
    feature(naked_functions)
)]
pub use {gfx_64, gui_64, math_64, sdl_64};

#[cfg(feature = "std")]
pub mod log {
    extern crate std;
    pub use log::*;

    pub fn init() {
        static LOGGER: Log = Log;
        log::set_logger(&LOGGER)
            .map(|()| log::set_max_level(LevelFilter::Debug))
            .expect("failed to init logs");
        log::info!("Greetz");
    }

    struct Log;

    impl log::Log for Log {
        fn enabled(&self, metadata: &Metadata) -> bool {
            metadata.level() <= Level::Debug
        }

        fn log(&self, record: &Record) {
            if self.enabled(record.metadata()) {
                println!("[{:>5}] {}", record.level(), record.args());
            }
        }

        fn flush(&self) {}
    }
}

#[cfg(not(feature = "std"))]
pub mod min_build {
    #[lang = "eh_personality"]
    fn eh_personality() {}

    #[panic_handler]
    fn panic(_info: &core::panic::PanicInfo) -> ! {
        loop {}
    }

    #[no_mangle]
    #[naked]
    pub unsafe extern "C" fn _start() {
        use core::arch::asm;

        #[allow(dead_code)]
        extern "C" {
            fn exit(_: core::ffi::c_int);
        }

        asm!(
            "mov rdi, rsp",
            "call main",
            "mov rax, 0",
            "call exit",
            options(noreturn)
        )
    }
}
