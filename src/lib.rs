#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![feature(new_uninit)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr::null_mut;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn client_install() {
        // INIT
        let mut global_context = Box::<signal_context>::new_uninit();
        let mut global_context: Box<signal_context> = unsafe {
            let result = signal_context_create(&mut global_context.as_mut_ptr() as *mut _, null_mut());
            assert_eq!(result, 0);
            global_context.assume_init()
        };



        // UNINIT
        unsafe {
            signal_context_destroy(Box::into_raw(global_context) as *mut _);
        }
    }
}
