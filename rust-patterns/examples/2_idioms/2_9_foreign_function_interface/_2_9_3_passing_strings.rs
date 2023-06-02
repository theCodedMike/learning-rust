#![allow(dead_code)]

//! ## 传递字符串
//!
//! ### 描述
//! 当传递字符串给FFI函数时，有以下4点需要遵守的原则：
//! 1. 让拥有的字符串生命周期尽可能长
//! 2. 在转换时保持最小化unsafe区域代码
//! 3. 如果C语言代码会修改字符串数据，那么使用Vec类型而不是CString
//! 4. 除非外部函数的API需要字符串的所有权，否则不要传给被调用的函数
//!
//! ### 优点
//! 样例能保证下面三点：
//! 1. unsafe代码块尽可能的小
//! 2. CString生命周期足够长
//! 3. 类型转换时发生的错误能够尽早地传播出来
//!
//! ### 缺点
//!
//! ### 讨论
//!

pub mod unsafe_module {

    // other module content

    extern "C" {
        fn set_err(message: *const libc::c_char);
        fn get_err(buffer: *mut libc::c_char, size: libc::c_int) -> libc::c_int;
    }

    fn report_error_to_ffi<S: Into<String>>(err: S) -> Result<(), std::ffi::NulError> {
        let c_err = std::ffi::CString::new(err.into())?;

        unsafe {
            // SAFETY: calling an FFI whose documentation says the pointer is
            // const, so no modification should occur
            set_err(c_err.as_ptr());
        }

        Ok(())
        // The lifetime of c_err continues until here
    }

    fn get_error_from_ffi() -> Result<String, std::ffi::IntoStringError> {
        let mut buffer = vec![0_i8; 1024];
        unsafe {
            // SAFETY: calling an FFI whose documentation implies
            // that the input need only live as long as the call
            let written: usize = get_err(buffer.as_mut_ptr(), 1023) as usize;

            buffer.truncate(written + 1);
        }

        // !!!补充!!!
        let new_buffer = buffer.into_iter().map(|ch| ch as u8).collect::<Vec<u8>>();

        std::ffi::CString::new(new_buffer).unwrap().into_string()
    }

    /// Wrong Example
    ///
    /// 这样的代码会导致悬垂指针
    fn report_error<S: Into<String>>(err: S) -> Result<(), std::ffi::NulError> {
        unsafe {
            // SAFETY: whoops, this contains a dangling pointer!
            set_err(std::ffi::CString::new(err.into())?.as_ptr());
        }
        Ok(())
    }
}
