
#[cfg(target_os = "emscripten")]
pub mod emscripten {
    use std::cell::RefCell;
    use std::ptr::null_mut;
    use std::os::raw::c_void;
    use emscripten_sys::{emscripten_set_main_loop, emscripten_async_wget, emscripten_async_wget_data};

    thread_local!(static MAIN_LOOP_CALLBACK: RefCell<*mut c_void> = RefCell::new(null_mut()));

    pub fn set_main_loop_callback<F>(callback: F) where F: FnMut() {
        MAIN_LOOP_CALLBACK.with(|log| {
            *log.borrow_mut() = &callback as *const _ as *mut c_void;
        });

        unsafe { emscripten_set_main_loop(Some(wrapper::<F>), 0, 1); }

        unsafe extern "C" fn wrapper<F>() where F: FnMut() {
            MAIN_LOOP_CALLBACK.with(|z| {
                let closure = *z.borrow_mut() as *mut F;
                (*closure)();
            });
        }
    }

    thread_local!(static ON_LOAD_CALLBACK: RefCell<*mut c_void> = RefCell::new(null_mut()));

    pub fn async_wget_data<F, E>(name: &str, on_load: F, on_error: E) where F: FnMut(String), E: FnOnce(String)
    {
        ON_LOAD_CALLBACK.with(|log| {
            *log.borrow_mut() = &on_load as *const _ as *mut c_void;
        });

        unsafe {
            let out = 0 as *const ::std::os::raw::c_char;
            emscripten_async_wget(name.as_ptr() as *const ::std::os::raw::c_char,
                                       out,
                                       Some(str_callback_wrapper::<F>),
                                       Some(str_callback_wrapper::<F>));
        }

        unsafe extern "C" fn str_callback_wrapper<F>(arg1: *const ::std::os::raw::c_char) where F: FnMut(String)
        {
            ON_LOAD_CALLBACK.with(|z| {
                let closure = *z.borrow_mut() as *mut F;
                (*closure)(String::from("Hej"));
            });
        }
    }

    /*thread_local!(static ON_LOAD_CALLBACK: RefCell<*mut c_void> = RefCell::new(null_mut()));

    pub fn async_wget_data<F>(name: &str, on_load: F, on_error: F) where F: FnOnce(String)
    {
        ON_LOAD_CALLBACK.with(|log| {
            *log.borrow_mut() = &on_load as *const _ as *mut c_void;
        });

        unsafe {
            emscripten_async_wget_data(name.as_ptr() as *const ::std::os::raw::c_char,
                                       0 as *mut ::std::os::raw::c_void,
                                       Some(on_load_wrapper::<F>),
                                       on_error);
        }

        unsafe extern "C" fn on_load_wrapper<F>(arg1: *mut ::std::os::raw::c_void, arg2: *mut ::std::os::raw::c_void, arg3: ::std::os::raw::c_int) where F: FnOnce(String)
        {
            ON_LOAD_CALLBACK.with(|z| {
                let closure = *z.borrow_mut() as *mut F;
                (*closure)(String::from("Hej"));
            });
        }
    }*/
}
