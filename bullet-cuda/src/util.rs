use std::ffi::c_void;

use crate::bindings::{
    cublasCreate_v2, cublasHandle_t, cudaDeviceSynchronize, cudaError, cudaFree, cudaGetLastError,
    cudaMalloc, cudaMemcpy, cudaMemcpyKind, cudaMemset, cudaGetDeviceCount, cudaGetDeviceProperties_v2,
};

#[macro_export]
macro_rules! catch {
    ($func:expr, $caller:expr) => {
        let err = unsafe { $func };
        if err != cudaError::cudaSuccess {
            panic!("{}: {:?}", $caller, err);
        }
    };
    ($func:expr) => {
        catch!($func, "synchronise")
    };
}

pub fn device_name() -> String {
    use std::ffi::CStr;
    let mut num = 0;
    catch!(cudaGetDeviceCount(&mut num));
    assert!(num >= 1);
    let mut props = bullet_core::util::boxed_and_zeroed();
    catch!(cudaGetDeviceProperties_v2(&mut *props, 0));

    let props_ptr = props.name.as_ptr();
    let props_len = props.name.len();

    unsafe {
        let name = std::slice::from_raw_parts(props_ptr.cast(), props_len);
        let cstr = CStr::from_bytes_until_nul(name).unwrap();
        let my_str = cstr.to_str().unwrap();
        my_str.to_string()
    }
}

pub fn create_cublas_handle() -> cublasHandle_t {
    let mut handle: cublasHandle_t = std::ptr::null_mut();
    unsafe {
        cublasCreate_v2((&mut handle) as *mut cublasHandle_t);
    }
    handle
}

pub fn device_synchronise() {
    catch!(cudaDeviceSynchronize());
}

pub fn panic_if_device_error(msg: &str) {
    catch!(cudaGetLastError(), msg);
}

pub fn malloc<T>(num: usize) -> *mut T {
    let size = num * std::mem::size_of::<T>();
    let mut grad = std::ptr::null_mut::<T>();
    let grad_ptr = (&mut grad) as *mut *mut T;

    assert!(!grad_ptr.is_null(), "null pointer");

    catch!(cudaMalloc(grad_ptr.cast(), size), "malloc");
    catch!(cudaDeviceSynchronize());

    grad
}

/// # Safety
/// Need to make sure not to double free.
pub unsafe fn free(ptr: *mut c_void) {
    catch!(cudaFree(ptr));
}

pub fn calloc<T>(num: usize) -> *mut T {
    let size = num * std::mem::size_of::<T>();
    let grad = malloc(num);
    catch!(cudaMemset(grad as *mut c_void, 0, size), "memset");
    catch!(cudaDeviceSynchronize());

    grad
}

pub fn set_zero<T>(ptr: *mut T, num: usize) {
    catch!(
        cudaMemset(ptr.cast(), 0, num * std::mem::size_of::<T>()),
        "memset"
    );
    catch!(cudaDeviceSynchronize());
}

pub fn copy_to_device<T>(dest: *mut T, src: *const T, amt: usize) {
    catch!(
        cudaMemcpy(
            dest.cast(),
            src.cast(),
            amt * std::mem::size_of::<T>(),
            cudaMemcpyKind::cudaMemcpyHostToDevice
        ),
        "memcpy"
    );
    catch!(cudaDeviceSynchronize());
}

pub fn copy_from_device<T>(dest: *mut T, src: *const T, amt: usize) {
    catch!(
        cudaMemcpy(
            dest.cast(),
            src.cast(),
            amt * std::mem::size_of::<T>(),
            cudaMemcpyKind::cudaMemcpyDeviceToHost
        ),
        "memcpy"
    );
    catch!(cudaDeviceSynchronize());
}
