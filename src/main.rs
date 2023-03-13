#![feature(const_cstr_methods)]
#![cfg_attr(
    not(feature = "std"),
    no_std,
    no_main,
    feature(lang_items),
    feature(naked_functions),
    feature(default_alloc_error_handler)
)]
use {
    core::ffi::{c_char, CStr},
    gfx_64::{
        resource::{
            mesh::{Mesh, Topology, Usage},
            program::Program,
            shader::{POS2D_TEX2D, TEX2D},
        },
        Resource, Target, SWAP_CHAIN,
    },
    gui_64::text::TextSystem,
    sdl_64::{
        event::{Event, EventChannel, KeyCode},
        window::Window,
    },
};

#[cfg(feature = "std")]
use _64::log;
#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
use _64::min_build::*;

const NAME_PTR: *const c_char = {
    const BYTES: &[u8] = b"Hello, world!\0";
    BYTES.as_ptr().cast()
};
const NAME: &CStr = unsafe { CStr::from_ptr(NAME_PTR) };
const WIDTH: i32 = 1920;
const HEIGHT: i32 = 1080;

#[cfg_attr(not(feature = "std"), no_mangle)]
pub fn main() {
    #[cfg(feature = "std")]
    log::init();

    #[cfg(not(feature = "std"))]
    mem_64::init();

    let window = Window::new(&NAME, 1920, 1080).expect("window creation failed");

    let text = TextSystem::default();

    let mut greets = gui_64::text::Text::new([1920, 1080]);
    greets.update("Greetz!\n  it builds:D");
    text.draw(&greets, [1920, 1080], 0, 10.0);

    let tex_quad = Mesh::new(
        &[
            ([-1.0, 1.0], [0.0, 1.0]),
            ([1.0, 1.0], [1.0, 1.0]),
            ([-1.0, -1.0], [0.0, 0.0]),
            ([1.0, -1.0], [1.0, 0.0]),
        ],
        Usage::StaticDraw,
        Topology::TriStrip,
    );

    let glyph_prog = Program::new(POS2D_TEX2D, TEX2D);
    glyph_prog.bind();

    let mut events = EventChannel;
    events.text_input(true);

    let mut edit = true;
    loop {
        #[cfg(feature = "std")]
        log::set_max_level(log::LevelFilter::Debug);

        match events.next() {
            Some(event) => match event {
                Event::Quit => {
                    break;
                }

                Event::Keyboard { down, sym, .. } if sym == KeyCode::CapsLock && down => {
                    edit = !edit;
                    #[cfg(feature = "std")]
                    log::debug!("Toggling edit mode to {}", edit);
                }

                _ => {}
            },

            None => {}
        };

        #[cfg(feature = "std")]
        log::set_max_level(log::LevelFilter::Off);

        SWAP_CHAIN.bind();
        SWAP_CHAIN.clear_color([0.0, 0.0, 0.0, 1.0]);
        SWAP_CHAIN.viewport([0, 0], [WIDTH, HEIGHT]);

        if edit {
            #[cfg(feature = "std")]
            log::set_max_level(log::LevelFilter::Debug);
            #[cfg(feature = "std")]
            log::debug!("Drawing UI");
            #[cfg(feature = "std")]
            log::set_max_level(log::LevelFilter::Off);

            greets.view().bind();
            tex_quad.draw();
        }
        window.swap();
    }

    #[cfg(not(feature = "std"))]
    unsafe {
        extern "C" {
            fn exit(status: u32);
        }

        exit(0);
    }
}
