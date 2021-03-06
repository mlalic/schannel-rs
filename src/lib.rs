//! Bindings to the Windows SChannel APIs.
#![warn(missing_docs)]
#![allow(non_upper_case_globals)]

extern crate advapi32;
extern crate crypt32;
extern crate kernel32;
extern crate secur32;
extern crate winapi;

#[macro_use]
extern crate lazy_static;

use std::ptr;

use key_handle::KeyHandle;

pub mod cert_context;
pub mod cert_store;
/* pub */ mod ctl_context;
pub mod key_handle;
pub mod schannel_cred;
pub mod tls_stream;

mod context_buffer;
mod security_context;

#[cfg(test)]
mod test;

const ACCEPT_REQUESTS: winapi::c_ulong =
    winapi::ASC_REQ_ALLOCATE_MEMORY | winapi::ASC_REQ_CONFIDENTIALITY |
    winapi::ASC_REQ_SEQUENCE_DETECT | winapi::ASC_REQ_STREAM |
    winapi::ASC_REQ_REPLAY_DETECT;

const INIT_REQUESTS: winapi::c_ulong =
    winapi::ISC_REQ_CONFIDENTIALITY | winapi::ISC_REQ_INTEGRITY | winapi::ISC_REQ_REPLAY_DETECT |
    winapi::ISC_REQ_SEQUENCE_DETECT | winapi::ISC_REQ_MANUAL_CRED_VALIDATION |
    winapi::ISC_REQ_ALLOCATE_MEMORY | winapi::ISC_REQ_STREAM;

trait Inner<T> {
    unsafe fn from_inner(t: T) -> Self;

    fn as_inner(&self) -> T;

    fn get_mut(&mut self) -> &mut T;
}

trait KeyHandlePriv {
    fn new(handle: winapi::HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, spec: winapi::DWORD) -> KeyHandle;
}

unsafe fn secbuf(buftype: winapi::c_ulong,
                 bytes: Option<&mut [u8]>) -> winapi::SecBuffer {
    let (ptr, len) = match bytes {
        Some(bytes) => (bytes.as_mut_ptr(), bytes.len() as winapi::c_ulong),
        None => (ptr::null_mut(), 0),
    };
    winapi::SecBuffer {
        BufferType: buftype,
        cbBuffer: len,
        pvBuffer: ptr as *mut winapi::c_void,
    }
}

unsafe fn secbuf_desc(bufs: &mut [winapi::SecBuffer]) -> winapi::SecBufferDesc {
    winapi::SecBufferDesc {
        ulVersion: winapi::SECBUFFER_VERSION,
        cBuffers: bufs.len() as winapi::c_ulong,
        pBuffers: bufs.as_mut_ptr(),
    }
}
