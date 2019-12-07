use libchezscheme as chezscheme;
use std::ffi::CString;

#[macro_export]
macro_rules! is_svalue {
    ( $x:expr, $y:expr ) => {
	if $x == $y {
	    true
	} else {
	    false
	}
    }
}

pub const SVOID: u64 = 0x3E;
pub const SEOF_OBJECT: u64 = 0x36;

macro_rules! eof_objectp { ( $x:expr ) => (is_svalue!($x, bindings::SEOF_OBJECT)) }

macro_rules! c_char {
     ( $x:expr ) => {
	 CString::new($x).unwrap().as_ptr()     
     }
}

pub fn ptr_to_u64(ptr: chezscheme::ptr) -> u64 {
    ptr as u64
}

pub fn u64_to_ptr(u: u64) -> chezscheme::ptr {
    u as chezscheme::ptr
}

pub fn string(string: &str) -> u64 {
    unsafe {
	ptr_to_u64(chezscheme::Sstring(c_char!(string)))
    }
}

unsafe extern "C" fn abnormal_exit() {
    panic!("chezscheme abnormal exit")
}

unsafe extern "C" fn custom_init() {}

pub fn scheme_init(exit_fn: Option< unsafe extern fn()>) {
    let exit_fn = if let Some(_f) = exit_fn {
	exit_fn
    } else {
	Some(abnormal_exit as unsafe extern fn())
    };
    unsafe {
	chezscheme::Sscheme_init(exit_fn);
    }
}

pub fn register_boot_file(path: &str) {
    unsafe {
	chezscheme::Sregister_boot_file(c_char!(path));
    }
}

pub fn build_heap(path: &str, init_fn: Option< unsafe extern fn()>) {
    let init_fn = if let Some(_f) = init_fn {
	init_fn
    } else {
	Some(custom_init as unsafe extern fn())
    };
    unsafe {
	chezscheme::Sbuild_heap(c_char!(path), init_fn);
    }
}

fn string_to_symbol_ptr(string: &str) -> chezscheme::ptr {
    unsafe {
	chezscheme::Sstring_to_symbol(c_char!(string))
    }
}

pub fn string_to_symbol(string: &str) -> u64 {
    ptr_to_u64(string_to_symbol_ptr(string))
}

pub fn call0(fn_name: &str) -> u64 {
    unsafe {
	ptr_to_u64(chezscheme::Scall0(
	    chezscheme::Stop_level_value(
		string_to_symbol_ptr(fn_name))))
    }
}

pub fn call1_string(fn_name: &str, arg: &str) -> u64 {
    unsafe {
	ptr_to_u64(chezscheme::Scall1(
	    chezscheme::Stop_level_value(
		string_to_symbol_ptr(fn_name)),
		chezscheme::Sstring(c_char!(arg))))
    }
}

pub fn call1_u64(fn_name: &str, arg: u64) -> u64 {
    unsafe {
	ptr_to_u64(chezscheme::Scall1(
	    chezscheme::Stop_level_value(
		string_to_symbol_ptr(fn_name)),
		u64_to_ptr(arg)))
    }
}

pub fn scheme_deinit() {
    unsafe {
	chezscheme::Sscheme_deinit()
    }
}

pub fn flonum(f: f64) -> u64 {
    unsafe {
	ptr_to_u64(chezscheme::Sflonum(f))
    }
}
