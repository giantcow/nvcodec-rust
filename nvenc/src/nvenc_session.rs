use std::ffi::c_void;

use nvenc_sys::ffi::{self, NV_ENC_DEVICE_TYPE};

use crate::{nvenc_api::NvEncApi, NvEncInitializeParams, NvEncResult};

pub struct NvEncSession {
    session: *mut c_void,
}

pub struct NvEncSessionBuilder {
    api: NvEncApi,
    device_type: NV_ENC_DEVICE_TYPE,
    device: Option<*mut c_void>,
    init_params: NvEncInitializeParams,
}

impl NvEncSessionBuilder {
    pub fn new(nvenc_api: NvEncApi, init_params: NvEncInitializeParams) -> Self {
        Self {
            api: nvenc_api,
            device_type: 0,
            device: None,
            init_params,
        }
    }

    pub fn with_cuda(mut self, cuda_ctx: *mut c_void) -> Self {
        self.device_type = ffi::_NV_ENC_DEVICE_TYPE_NV_ENC_DEVICE_TYPE_CUDA;
        self.device = Some(cuda_ctx);
        self
    }

    pub fn with_opengl(mut self, opengl_device: *mut c_void) -> Self {
        self.device_type = ffi::_NV_ENC_DEVICE_TYPE_NV_ENC_DEVICE_TYPE_OPENGL;
        self.device = Some(opengl_device);
        self
    }

    pub fn build(mut self) -> NvEncResult<NvEncSession> {
        let session = self.api.open_encode_session_ex(
            self.device_type,
            &self
                .device
                .expect("with_*_device(...) to have been called once"),
        )?;

        self.api
            .initialize_encoder(session, &mut self.init_params)?;

        Ok(NvEncSession { session })
    }
}

#[cfg(test)]
mod test {
    use crate::nvenc_api::NvEncApiBuilder;

    use super::*;

    #[test]
    fn build_err12_with_default_params() {
        let cuda_ctx = cust::quick_init().unwrap();

        assert_eq!(
            Some(crate::NvEncApiError::from(12)),
            NvEncSessionBuilder::new(
                NvEncApiBuilder::new().build().unwrap(),
                NvEncInitializeParams::default(),
            )
            .with_cuda(cuda_ctx.as_raw() as *mut _ as *mut c_void)
            .build()
            .err()
        )
    }

    #[test]
    fn builds() {
        let mut init_params = NvEncInitializeParams::default();
        init_params.set_encode_guid(ffi::NV_ENC_CODEC_H264_GUID);
        init_params.set_preset_guid(ffi::NV_ENC_PRESET_P1_GUID);

        let _session =
            NvEncSessionBuilder::new(NvEncApiBuilder::new().build().unwrap(), init_params)
                .with_cuda(cust::quick_init().unwrap().as_raw() as *mut _ as *mut c_void)
                .build()
                .unwrap();
    }
}
