#![crate_type = "cdylib"]
#![no_std]

#[panic_handler]
unsafe fn panic(_panic: &core::panic::PanicInfo) -> ! { ExitProcess(1); loop { } }

#[no_mangle]
unsafe extern "system" fn _DllMainCRTStartup(_: *const u8, _: u32, _: *const u8) -> u32 { 1 }

#[link(name = "kernel32")]
#[allow(non_snake_case)]
extern "stdcall" {
    fn ExitProcess(exit_code: u32) -> ();
}

#[link(name = "shlwapi")]
#[allow(non_snake_case)]
extern "stdcall" {
    fn PathCanonicalizeW(path_out: *mut u8, path: *const u8) -> bool;
    fn PathCombineW(path_out: *mut u8, dir: *const u8, file: *const u8) -> usize;
}

const MAX_PATH: usize = 256;
const S_OK: u32 = 0;
const E_INVALIDARG: u32 = 0x80070057;
const PATHCCH_NONE: u32 = 0; 


#[no_mangle]
pub unsafe extern "stdcall" fn PathCchCanonicalizeEx(path_out_buf: *mut u8, buf_size: usize, path: *const u8, flags: u32) -> u32 {
	if flags != PATHCCH_NONE || buf_size < MAX_PATH { return E_INVALIDARG; }
	let success = PathCanonicalizeW(path_out_buf, path);
	return if success { S_OK } else { E_INVALIDARG };
}

#[no_mangle]
pub unsafe extern "stdcall" fn PathCchCombineEx(path_out_buf: *mut u8, buf_size: usize, path: *const u8, path_more: *const u8, flags: u32) -> u32 {
	if flags != PATHCCH_NONE || buf_size < MAX_PATH { return E_INVALIDARG; }
	let str_ptr = PathCombineW(path_out_buf, path, path_more);
	return if str_ptr > 0 { S_OK } else { E_INVALIDARG };
}