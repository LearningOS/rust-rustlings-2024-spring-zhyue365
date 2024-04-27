unsafe fn modify_by_address(address: usize) {
    // SAFETY: The caller must ensure that `address` is a valid pointer
    // to a `u32` value and that it is safe to dereference and mutate it.
    unsafe {
        // Cast the address to a mutable pointer to u32 and dereference it
        // to modify the value.
        // Note: In real-world usage, you should be very careful with raw pointer
        // manipulations and ensure that they are safe according to the safety
        // contract outlined above.
        *(address as *mut u32) = 0xAABBCCDD;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;

        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert_eq!(t, 0xAABBCCDD);
    }
}