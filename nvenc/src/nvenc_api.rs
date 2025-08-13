use std::ffi::c_void;

use nvenc_sys::ffi::{
    self, NVENCAPI_VERSION, NV_ENCODE_API_FUNCTION_LIST_VER, NV_ENC_DEVICE_TYPE,
    NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS, NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS_VER,
    _NVENCSTATUS_NV_ENC_SUCCESS,
};

use crate::{NvEncApiError, NvEncInitializeParams, NvEncResult};

pub struct NvEncApi {
    inner: ffi::NV_ENCODE_API_FUNCTION_LIST,
}

#[derive(Default)]
pub struct NvEncApiBuilder {}

impl NvEncApiBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> NvEncResult<NvEncApi> {
        let mut nv_funcs = unsafe { std::mem::zeroed::<ffi::NV_ENCODE_API_FUNCTION_LIST>() };
        nv_funcs.version = NV_ENCODE_API_FUNCTION_LIST_VER;
        let res_code = unsafe { ffi::NvEncodeAPICreateInstance(&mut nv_funcs) };
        if res_code != 0 {
            Err(NvEncApiError::from(res_code))
        } else {
            Ok(NvEncApi { inner: nv_funcs })
        }
    }
}

impl NvEncApi {
    pub(crate) fn open_encode_session_ex(
        &mut self,
        device_type: NV_ENC_DEVICE_TYPE,
        device: &*mut c_void,
    ) -> NvEncResult<*mut c_void> {
        let mut session = std::ptr::null_mut::<c_void>();

        let mut open_session =
            unsafe { std::mem::zeroed::<NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS>() };
        open_session.version = NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS_VER;
        open_session.apiVersion = NVENCAPI_VERSION;
        open_session.deviceType = device_type;
        open_session.device = *device;

        let res = unsafe {
            self.inner
                .nvEncOpenEncodeSessionEx
                .expect("nvEncOpenEncodeSessionEx not loaded")(
                &mut open_session, &mut session
            )
        };
        if res != _NVENCSTATUS_NV_ENC_SUCCESS {
            // Err(NvEncApiError::from(res))
            Err(res.into())
        } else {
            Ok(session)
        }
    }

    pub(crate) fn initialize_encoder(
        &mut self,
        session_handle: *mut c_void,
        params: &mut NvEncInitializeParams,
    ) -> NvEncResult<()> {
        let res = unsafe {
            self.inner
                .nvEncInitializeEncoder
                .expect("nvEncInitializeEncoder not loaded")(
                session_handle, params.as_raw()
            )
        };

        if res != 0 {
            Err(res.into())
        } else {
            Ok(())
        }
    }
}
