use std::{ops::Deref, os::raw::c_void};

use crate::{
    cef_ref::{CefRefData, CefStruct},
    validated::Validated,
};

use super::{context::V8ContextEntered, value::V8Value};

pub struct V8ArrayBuffer(pub(super) Validated<chromium_sys::cef_v8value_t>);

impl V8ArrayBuffer {
    pub fn new(buffer: Vec<u8>, _context_entered: &V8ContextEntered) -> Self {
        let release_callback = V8ArrayBufferReleaseCallback::Delete {
            buffer_len: buffer.len(),
            buffer_cap: buffer.capacity(),
        };

        let buffer = buffer.leak();
        let inner = unsafe {
            chromium_sys::cef_v8value_create_array_buffer(
                buffer.as_mut_ptr() as *mut c_void,
                buffer.len(),
                CefRefData::new_ptr(release_callback),
            )
        };

        Self(Validated::new(inner))
    }

    /// Creates a new array buffer from raw pointer. It can be only created while in context.
    /// The buffer's memory is shared with V8 engine.
    ///
    /// # Safety
    /// Make sure the pointer is valid. Invalid pointer can cause undefined behavior.
    pub unsafe fn from_ptr(
        ptr: *mut u8,
        ptr_len: usize,
        _context_entered: &V8ContextEntered,
    ) -> Self {
        // We do not delete the buffer because it's not owned by this function
        let release_callback = V8ArrayBufferReleaseCallback::DoNotDelete;
        let inner = unsafe {
            chromium_sys::cef_v8value_create_array_buffer(
                ptr as *mut c_void,
                ptr_len,
                CefRefData::new_ptr(release_callback),
            )
        };

        Self(Validated::new(inner))
    }
}

impl From<V8ArrayBuffer> for V8Value {
    fn from(value: V8ArrayBuffer) -> Self {
        Self::ArrayBuffer(value)
    }
}

enum V8ArrayBufferReleaseCallback {
    Delete {
        buffer_len: usize,
        buffer_cap: usize,
    },
    DoNotDelete,
}

impl CefStruct for V8ArrayBufferReleaseCallback {
    type CefType = chromium_sys::cef_v8array_buffer_release_callback_t;

    fn cef_data(&self) -> Self::CefType {
        chromium_sys::cef_v8array_buffer_release_callback_t {
            base: unsafe { std::mem::zeroed() },
            release_buffer: Some(Self::release_buffer),
        }
    }

    fn base_mut(cef_data: &mut Self::CefType) -> &mut chromium_sys::cef_base_ref_counted_t {
        &mut cef_data.base
    }
}

impl V8ArrayBufferReleaseCallback {
    extern "C" fn release_buffer(
        self_: *mut chromium_sys::cef_v8array_buffer_release_callback_t,
        buffer: *mut c_void,
    ) {
        unsafe {
            let self_ref = CefRefData::<Self>::from_cef(self_);
            match (*self_ref).deref() {
                V8ArrayBufferReleaseCallback::Delete {
                    buffer_len,
                    buffer_cap,
                } => {
                    Vec::from_raw_parts(buffer, *buffer_len, *buffer_cap);
                }
                V8ArrayBufferReleaseCallback::DoNotDelete => {}
            };
        }
    }
}
