
#[macro_use]
mod bindings;

pub fn repl(petite_boot: &str, scheme_boot: &str) {
    bindings::scheme_init(None);
    bindings::register_boot_file(petite_boot);
    bindings::register_boot_file(scheme_boot);
    bindings::build_heap("./", None);

    loop {
	bindings::call1_string("display", "* ");
	let p = bindings::call0("read");
	if eof_objectp!(p) { break }
	let p = bindings::call1_u64("eval", p);
	if !is_svalue!(p, bindings::SVOID) {
	    bindings::call1_u64("pretty-print", p);
	}
    }
    bindings::scheme_deinit()
}

pub fn flonum() -> u64 {
    bindings::flonum(0.22_f64)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
