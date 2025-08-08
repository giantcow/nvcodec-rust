#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    /// Macro to generate per-structure version for use with API.
    // Header definition:
    // #define NVENCAPI_STRUCT_VERSION(ver) ((uint32_t)NVENCAPI_VERSION | ((ver)<<16) | (0x7 << 28))
    pub const fn NVENCAPI_STRUCT_VERSION(ver: u32) -> u32 {
        NVENCAPI_VERSION | ((ver << 16) | 0x7 << 28)
    }

    pub const NV_ENCODE_API_FUNCTION_LIST_VER: u32 = NVENCAPI_STRUCT_VERSION(2);
    pub const NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS_VER: u32 = NVENCAPI_STRUCT_VERSION(1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_create_api_instance() {
        unsafe {
            let mut nv_func_list = std::mem::zeroed::<ffi::NV_ENCODE_API_FUNCTION_LIST>();
            nv_func_list.version = ffi::NV_ENCODE_API_FUNCTION_LIST_VER;

            let res = ffi::NvEncodeAPICreateInstance(&mut nv_func_list);
            assert_eq!(ffi::_NVENCSTATUS_NV_ENC_SUCCESS, res);
        }
    }
}
