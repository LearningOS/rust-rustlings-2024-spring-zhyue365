// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.

/// # Safety

unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {

    unsafe { Box::from_raw(ptr) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: None });


        let raw_ptr = Box::into_raw(data);
        let ret = unsafe { raw_pointer_to_box(raw_ptr) };

        let _raw_ptr_again = Box::into_raw(ret);


        let ptr_2 = &ret.a as *const u128 as usize;

        assert!(ptr_2 == 1);


    }
}