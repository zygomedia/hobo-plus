#![feature(proc_macro_hygiene, stmt_expr_attributes)]

use hobo::prelude::*;
#[allow(unused_imports)] use clown::{clown, honk, slip};
pub use entity_ext::AsEntityExt;
pub use element_ext::{children_diff::{ChildrenDiff, ChildrenDiffConfig, ChildrenDiffConfigBuilder, ChildrenDiffElementExt, ItemMapping}, AsElementExt, FontTag, Clicked};
pub use html_ext::{AExt, Toggleable, ToggleableExt};
pub use svg::xml_to_svg;
pub use __svgs as svgs;

mod html_ext;
mod entity_ext;
mod element_ext;
pub mod file_select;
pub mod svg;
pub mod socket;

pub fn window() -> web_sys::Window { web_sys::window().expect("no window") }
pub fn document() -> web_sys::Document { window().document().expect("no document") }

fn closure_mut<T: wasm_bindgen::convert::FromWasmAbi + 'static> (closure: impl FnMut(T) + 'static) -> Closure<dyn FnMut(T)> {
	Closure::wrap(Box::new(closure) as Box<dyn FnMut(T) + 'static>)
}

pub fn animation(f: impl FnMut(f64) -> bool + 'static) {
	animation_with_window(&window(), f);
}

// run a function every frame until it returns false
// fn argument is delta milliseconds
// skips the first frame immediately after because it's not possible to calculate time delta
#[expect(clippy::clone_on_ref_ptr)]
pub fn animation_with_window(window: &web_sys::Window, mut f: impl FnMut(f64) -> bool + 'static) {
	use std::{cell::RefCell, rc::Rc};

	// this weird refcelling is necessary for "recursion"
	let cb = Rc::new(RefCell::new(None as Option<Closure<dyn FnMut(f64) + 'static>>));
	let mut last_timestamp = None;
	*cb.borrow_mut() = Some(Closure::wrap(Box::new(#[clown] |timestamp| {
		let cb = Rc::clone(&honk!(cb));
		let window = honk!(window).clone();

		if window.closed().unwrap_or(true) { let _drop = cb.borrow_mut().take(); return; }
		let Some(last_timestamp) = last_timestamp.as_mut() else {
			window.request_animation_frame(cb.borrow().as_ref().unwrap().as_ref().unchecked_ref()).unwrap();
			last_timestamp = Some(timestamp);
			return;
		};
		let delta_t = timestamp - *last_timestamp;
		*last_timestamp = timestamp;
		if f(delta_t) {
			window.request_animation_frame(cb.borrow().as_ref().unwrap().as_ref().unchecked_ref()).unwrap();
		} else {
			let _drop = cb.borrow_mut().take();
		}
	}) as Box<dyn FnMut(f64) + 'static>));
	window.request_animation_frame(cb.borrow().as_ref().unwrap().as_ref().unchecked_ref()).unwrap();
}

// basically just copy of rust std's dbg!
#[macro_export]
macro_rules! __dbg {
	() => { log::info!("[{}:{}]", file!(), line!()); };
	($val:expr) => { match $val { tmp => { log::info!("[{}:{}] {} = {:#?}", file!(), line!(), stringify!($val), &tmp); tmp } } };
	($val:expr,) => { $crate::dbg!($val) };
	($($val:expr),+ $(,)?) => { ($($crate::dbg!($val)),+,) };
}
