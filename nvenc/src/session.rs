use std::ffi::c_void;

use nvenc_sys::ffi::{self, NV_ENC_DEVICE_TYPE};

use crate::{api::NvEncApi, NvEncInitializeParams, NvEncResult};

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
    // TODO: Fix these tests, they're causing cargo to hang

    // use crate::{
    //     api::NvEncApiBuilder, guids, nv_enc_codec, ChromaFormat, NvEncApiError, NvEncConfig,
    // };
    //
    // use super::*;

    // #[test]
    // fn build_err12_with_default_params() {
    //     let cuda_ctx = cust::quick_init().unwrap();
    //
    //     assert_eq!(
    //         Some(NvEncApiError::from(12)),
    //         NvEncSessionBuilder::new(
    //             NvEncApiBuilder::new().build().unwrap(),
    //             NvEncInitializeParams::default(),
    //         )
    //         .with_cuda(cuda_ctx.as_raw() as *mut _ as *mut c_void)
    //         .build()
    //         .err()
    //     )
    // }
    //
    // #[test]
    // fn builds_session_minimal_h264() {
    //     let mut init_params = NvEncInitializeParams::default();
    //     init_params.set_encode_resolution(1920, 1080);
    //     init_params.set_encode_guid(guids::codecs::NV_ENC_CODEC_H264_GUID);
    //
    //     let h264_config = nv_enc_codec::H264::new(ChromaFormat::YUV420);
    //
    //     let mut encode_config = NvEncConfig::default();
    //     encode_config.set_encode_codec_config(h264_config.into());
    //
    //     init_params.set_encode_config(&mut encode_config);
    //
    //     let session =
    //         NvEncSessionBuilder::new(NvEncApiBuilder::new().build().unwrap(), init_params)
    //             .with_cuda(cust::quick_init().unwrap().as_raw() as *mut _ as *mut c_void)
    //             .build()
    //             .unwrap();
    //
    //     assert!(!std::ptr::eq(
    //         session.session,
    //         std::ptr::null::<std::ffi::c_void>()
    //     ));
    // }
}
