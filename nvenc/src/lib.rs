use std::fmt::{Debug, Display};

use nvenc_sys::ffi;

pub mod nvenc_api;
pub mod nvenc_session;

pub type NvEncResult<R> = Result<R, NvEncApiError>;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct NvEncApiError(u32);

impl NvEncApiError {
    pub fn get_details(&self) -> (&str, &str) {
        match self.0 {
            ffi::_NVENCSTATUS_NV_ENC_SUCCESS => ("NV_ENC_SUCCESS", "This indicates that API call returned with no errors."),
            ffi::_NVENCSTATUS_NV_ENC_ERR_NO_ENCODE_DEVICE => ("NV_ENC_ERR_INVALID_ENCODERDEVICE", "This indicates that no encode capable devices were detected."),
            ffi::_NVENCSTATUS_NV_ENC_ERR_UNSUPPORTED_DEVICE => ("NV_ENC_ERR_UNSUPPORTED_DEVICE", "This indicates that devices pass by the client is not supported."),
            ffi::_NVENCSTATUS_NV_ENC_ERR_INVALID_ENCODERDEVICE => ("NV_ENC_ERR_INVALID_ENCODERDEVICE", "This indicates that the encoder device supplied by the client is not valid."),
            ffi::_NVENCSTATUS_NV_ENC_ERR_INVALID_DEVICE => ("NV_ENC_ERR_INVALID_DEVICE", "This indicates that device passed to the API call is invalid."),
            ffi::_NVENCSTATUS_NV_ENC_ERR_DEVICE_NOT_EXIST => ("NV_ENC_ERR_DEVICE_NOT_EXIST", "This indicates that device passed to the API call is no longer available and needs to be reinitialized. The clients need to destroy the current encoder session by freeing the allocated input output buffers and destroying the device and create a new encoding session."),
            ffi::_NVENCSTATUS_NV_ENC_ERR_INVALID_PTR => ("NV_ENC_ERR_INVALID_PTR", "This indicates that one or more of the pointers passed to the API call is invalid."),
            ffi::_NVENCSTATUS_NV_ENC_ERR_INVALID_EVENT => ("NV_ENC_ERR_INVALID_EVENT", "This indicates that completion event passed in ::NvEncEncodePicture() call is invalid."),
            ffi::_NVENCSTATUS_NV_ENC_ERR_INVALID_PARAM => ("NV_ENC_ERR_INVALID_PARAM", "This indicates that one or more of the parameter passed to the API call is invalid."),
            ffi::_NVENCSTATUS_NV_ENC_ERR_INVALID_CALL => ("NV_ENC_ERR_INVALID_CALL", "This indicates that an API call was made in wrong sequence/order."),
            ffi::_NVENCSTATUS_NV_ENC_ERR_OUT_OF_MEMORY => ("NV_ENC_ERR_OUT_OF_MEMORY", "This indicates that the API call failed because it was unable to allocate enough memory to perform the requested operation."),
            ffi::_NVENCSTATUS_NV_ENC_ERR_ENCODER_NOT_INITIALIZED => ("NV_ENC_ERR_ENCODER_NOT_INITIALIZED", "This indicates that the encoder has not been initialized with ::NvEncInitializeEncoder() or that initialization has failed. The client cannot allocate input or output buffers or do any encoding related operation before successfully initializing the encoder."),
            ffi::_NVENCSTATUS_NV_ENC_ERR_UNSUPPORTED_PARAM => ("NV_ENC_ERR_UNSUPPORTED_PARAM", "This indicates that an unsupported parameter was passed by the client."),
            ffi::_NVENCSTATUS_NV_ENC_ERR_LOCK_BUSY => ("NV_ENC_ERR_LOCK_BUSY", "This indicates that the ::NvEncLockBitstream() failed to lock the output buffer. This happens when the client makes a non blocking lock call to access the output bitstream by passing NV_ENC_LOCK_BITSTREAM::doNotWait flag. This is not a fatal error and client should retry the same operation after few milliseconds."),
            ffi::_NVENCSTATUS_NV_ENC_ERR_NOT_ENOUGH_BUFFER => ("NV_ENC_ERR_NOT_ENOUGH_BUFFER", "This indicates that the size of the user buffer passed by the client is insufficient for the requested operation."),
            ffi::_NVENCSTATUS_NV_ENC_ERR_INVALID_VERSION => ("NV_ENC_ERR_INVALID_VERSION", "This indicates that an invalid struct version was used by the client."),
            ffi::_NVENCSTATUS_NV_ENC_ERR_MAP_FAILED => ("NV_ENC_ERR_MAP_FAILED", "This indicates that ::NvEncMapInputResource() API failed to map the client provided input resource."),
            ffi::_NVENCSTATUS_NV_ENC_ERR_NEED_MORE_INPUT => ("NV_ENC_ERR_NEED_MORE_INPUT", "This indicates encode driver requires more input buffers to produce an output bitstream. If this error is returned from ::NvEncEncodePicture() API, this is not a fatal error. If the client is encoding with B frames then, ::NvEncEncodePicture() API might be buffering the input frame for re-ordering. A client operating in synchronous mode cannot call ::NvEncLockBitstream() API on the output bitstream buffer if ::NvEncEncodePicture() returned the ::NV_ENC_ERR_NEED_MORE_INPUT error code. The client must continue providing input frames until encode driver returns ::NV_ENC_SUCCESS. After receiving ::NV_ENC_SUCCESS status the client can call ::NvEncLockBitstream() API on the output buffers in the same order in which it has called ::NvEncEncodePicture()."),
            ffi::_NVENCSTATUS_NV_ENC_ERR_ENCODER_BUSY => ("NV_ENC_ERR_ENCODER_BUSY", "This indicates that the HW encoder is busy encoding and is unable to encode the input. The client should call ::NvEncEncodePicture() again after few milliseconds."),
            ffi::_NVENCSTATUS_NV_ENC_ERR_EVENT_NOT_REGISTERD => ("NV_ENC_ERR_EVENT_NOT_REGISTERD", "This indicates that the completion event passed in ::NvEncEncodePicture() API has not been registered with encoder driver using ::NvEncRegisterAsyncEvent()."),
            ffi::_NVENCSTATUS_NV_ENC_ERR_GENERIC => ("NV_ENC_ERR_GENERIC", "This indicates that an unknown internal error has occurred."),
            ffi::_NVENCSTATUS_NV_ENC_ERR_INCOMPATIBLE_CLIENT_KEY => ("NV_ENC_ERR_INCOMPATIBLE_CLIENT_KEY", "This indicates that the client is attempting to use a feature that is not available for the license type for the current system."),
            ffi::_NVENCSTATUS_NV_ENC_ERR_UNIMPLEMENTED => ("NV_ENC_ERR_UNIMPLEMENTED", "This indicates that the client is attempting to use a feature that is not implemented for the current version."),
            ffi::_NVENCSTATUS_NV_ENC_ERR_RESOURCE_REGISTER_FAILED => ("NV_ENC_ERR_RESOURCE_REGISTER_FAILED", "This indicates that the ::NvEncRegisterResource API failed to register the resource."),
            ffi::_NVENCSTATUS_NV_ENC_ERR_RESOURCE_NOT_REGISTERED => ("NV_ENC_ERR_RESOURCE_NOT_REGISTERED", "This indicates that the client is attempting to unregister a resource that has not been successfully registered."),
            ffi::_NVENCSTATUS_NV_ENC_ERR_RESOURCE_NOT_MAPPED => ("NV_ENC_ERR_RESOURCE_NOT_MAPPED", "This indicates that the client is attempting to unmap a resource that has not been successfully mapped."),
            ffi::_NVENCSTATUS_NV_ENC_ERR_NEED_MORE_OUTPUT => ("NV_ENC_ERR_NEED_MORE_OUTPUT", "This indicates encode driver requires more output buffers to write an output bitstream. If this error is returned from ::NvEncRestoreEncoderState() API, this is not a fatal error. If the client is encoding with B frames then, ::NvEncRestoreEncoderState() API might be requiring the extra output buffer for accomodating overlay frame output in a separate buffer, for AV1 codec. In this case, client must call NvEncRestoreEncoderState() API again with NV_ENC_RESTORE_ENCODER_STATE_PARAMS::outputBitstream as input along with the parameters in the previous call. When operating in asynchronous mode of encoding, client must also specify NV_ENC_RESTORE_ENCODER_STATE_PARAMS::completionEvent."),
            _ => (
                "UNKNOWN",
                "Could not match the error code against a known NVENCSTATUS",
            ),
        }
    }
}

impl From<u32> for NvEncApiError {
    fn from(value: u32) -> Self {
        NvEncApiError(value)
    }
}

impl Debug for NvEncApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (err_name, err_desc) = self.get_details();
        write!(f, "{} (Code: {}) \"{}\"", err_name, self.0, err_desc)
    }
}

impl Display for NvEncApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (err_name, _) = self.get_details();
        write!(f, "{} ({})", err_name, self.0)
    }
}

pub struct NvEncInitializeParams(ffi::_NV_ENC_INITIALIZE_PARAMS);

impl Debug for NvEncInitializeParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params = &self.0;
        f.debug_struct("NvEncInitializeParams")
            .field("version", &params.version)
            .field("encodeGUID", &params.encodeGUID)
            .field("presetGUID", &params.presetGUID)
            .field("encodeWidth", &params.encodeWidth)
            .field("encodeHeight", &params.encodeHeight)
            .field("maxEncodeWidth", &params.maxEncodeWidth)
            .field("maxEncodeHeight", &params.maxEncodeHeight)
            .field("darWidth", &params.darWidth)
            .field("darHeight", &params.darHeight)
            .field("frameRateNum", &params.frameRateNum)
            .field("frameRateDen", &params.frameRateDen)
            .field("enableEncodeAsync", &params.enableEncodeAsync)
            .field("enablePTD", &params.enablePTD)
            .field("encodeConfig", &params.encodeConfig)
            .field("maxMEHintCountsPerBlock", &params.maxMEHintCountsPerBlock)
            .field("tuningInfo", &params.tuningInfo)
            .field("bufferFormat", &params.bufferFormat)
            .field("numStateBuffers", &params.numStateBuffers)
            .field("outputStatsLevel", &params.outputStatsLevel)
            .finish()
    }
}

impl NvEncInitializeParams {
    pub fn set_encode_resolution(&mut self, width: u32, height: u32) -> &mut Self {
        self.0.encodeWidth = width;
        self.0.encodeHeight = height;

        #[cfg(not(feature = "disable-assume-max-encode-width"))]
        {
            if self.0.maxEncodeWidth == 0 {
                self.0.maxEncodeWidth = self.0.encodeWidth;
            }

            if self.0.maxEncodeHeight == 0 {
                self.0.maxEncodeHeight = self.0.encodeWidth;
            }
        }

        self
    }

    pub fn set_encode_resolution_max(&mut self, width: u32, height: u32) -> &mut Self {
        self.0.maxEncodeWidth = width;
        self.0.maxEncodeHeight = height;
        self
    }

    pub fn set_dar_resolution(&mut self, width: u32, height: u32) -> &mut Self {
        self.0.darWidth = width;
        self.0.darHeight = height;
        self
    }

    pub fn set_framerate(&mut self, numerator: u32, denomenator: u32) -> &mut Self {
        self.0.frameRateNum = numerator;
        self.0.frameRateDen = denomenator;
        self
    }

    pub fn set_encode_guid(&mut self, encode_guid: ffi::GUID) -> &mut Self {
        self.0.encodeGUID = encode_guid;
        self
    }

    pub fn set_preset_guid(&mut self, preset_guid: ffi::GUID) -> &mut Self {
        self.0.presetGUID = preset_guid;
        self
    }

    pub fn set_encode_config(&mut self, encode_config: &mut NvEncConfig) -> &mut Self {
        self.0.encodeConfig = &mut encode_config.0 as *mut _;
        self
    }

    /// Enable or Disable the Picture Type Decision
    pub fn set_ptd(&mut self, should_enable: bool) -> &mut Self {
        if should_enable {
            self.0.enablePTD = 1;
        } else {
            self.0.enablePTD = 0;
        }

        self
    }

    pub fn as_raw(&mut self) -> &mut ffi::NV_ENC_INITIALIZE_PARAMS {
        &mut self.0
    }
}

impl Default for NvEncInitializeParams {
    fn default() -> Self {
        let mut params = Self(unsafe { std::mem::zeroed() });
        params.0.version = ffi::NV_ENC_INITIALIZE_PARAMS_VER;
        params
    }
}

pub struct NvEncConfig(ffi::NV_ENC_CONFIG);

impl Default for NvEncConfig {
    fn default() -> Self {
        let mut config = unsafe { std::mem::zeroed::<ffi::NV_ENC_CONFIG>() };
        config.version = ffi::NV_ENC_CONFIG_VER;
        Self(config)
    }
}

impl NvEncConfig {
    pub fn set_encode_codec_config(&mut self, enode_codec_config: NvEncCodecConfig) -> &mut Self {
        self.0.encodeCodecConfig = match enode_codec_config {
            NvEncCodecConfig::H264(mut config) => ffi::NV_ENC_CODEC_CONFIG {
                h264Config: config.as_mut().0,
            },
            NvEncCodecConfig::HEVC(mut config) => ffi::NV_ENC_CODEC_CONFIG {
                hevcConfig: config.as_mut().0,
            },
            NvEncCodecConfig::AV1(mut config) => ffi::NV_ENC_CODEC_CONFIG {
                av1Config: config.as_mut().0,
            },
            NvEncCodecConfig::H264MeOnly(mut config) => ffi::NV_ENC_CODEC_CONFIG {
                h264MeOnlyConfig: config.as_mut().0,
            },
            NvEncCodecConfig::HEVCMeOnly(mut config) => ffi::NV_ENC_CODEC_CONFIG {
                hevcMeOnlyConfig: config.as_mut().0,
            },
        };
        self
    }
}

pub enum NvEncCodecConfig {
    H264(Box<nv_enc_codec::H264>),
    HEVC(Box<nv_enc_codec::HEVC>),
    AV1(Box<nv_enc_codec::AV1>),
    H264MeOnly(Box<nv_enc_codec::H264MeOnly>),
    HEVCMeOnly(Box<nv_enc_codec::HEVCMeOnly>),
}

impl From<nv_enc_codec::H264> for NvEncCodecConfig {
    fn from(value: nv_enc_codec::H264) -> Self {
        Self::H264(Box::new(value))
    }
}

impl From<nv_enc_codec::HEVC> for NvEncCodecConfig {
    fn from(value: nv_enc_codec::HEVC) -> Self {
        Self::HEVC(Box::new(value))
    }
}

impl From<nv_enc_codec::AV1> for NvEncCodecConfig {
    fn from(value: nv_enc_codec::AV1) -> Self {
        Self::AV1(Box::new(value))
    }
}

impl From<nv_enc_codec::H264MeOnly> for NvEncCodecConfig {
    fn from(value: nv_enc_codec::H264MeOnly) -> Self {
        Self::H264MeOnly(Box::new(value))
    }
}

impl From<nv_enc_codec::HEVCMeOnly> for NvEncCodecConfig {
    fn from(value: nv_enc_codec::HEVCMeOnly) -> Self {
        Self::HEVCMeOnly(Box::new(value))
    }
}

// TODO: Make some proc macros to remove the amount of repeated statements
pub mod nv_enc_codec {
    #![allow(dead_code)] // TODO: Remove

    use super::*;

    pub struct H264(pub(super) ffi::NV_ENC_CONFIG_H264);
    impl BaseNvCodec for H264 {}
    impl Debug for H264 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("NV_ENC_CONFIG_H264")
                .field("level", &self.0.level)
                .field("idrPeriod", &self.0.idrPeriod)
                .field("separateColourPlaneFlag", &self.0.separateColourPlaneFlag)
                .field(
                    "disableDeblockingFilterIDC",
                    &self.0.disableDeblockingFilterIDC,
                )
                .field("numTemporalLayers", &self.0.numTemporalLayers)
                .field("spsId", &self.0.spsId)
                .field("ppsId", &self.0.ppsId)
                .field("adaptiveTransformMode", &self.0.adaptiveTransformMode) // Enum
                .field("fmoMode", &self.0.fmoMode) // Enum
                .field("bdirectMode", &self.0.bdirectMode) // Enum
                .field("entropyCodingMode", &self.0.entropyCodingMode) // Enum
                .field("stereoMode", &self.0.stereoMode) // Enum
                .field("intraRefreshPeriod", &self.0.intraRefreshPeriod)
                .field("intraRefreshCnt", &self.0.intraRefreshCnt)
                .field("maxNumRefFrames", &self.0.maxNumRefFrames)
                .field("sliceMode", &self.0.sliceMode)
                .field("sliceModeData", &self.0.sliceModeData)
                .field("h264VUIParameters", &self.0.h264VUIParameters) // Enum
                .field("ltrNumFrames", &self.0.ltrNumFrames)
                .field("ltrTrustMode", &self.0.ltrTrustMode)
                .field("chromaFormatIDC", &self.0.chromaFormatIDC)
                .field("maxTemporalLayers", &self.0.maxTemporalLayers)
                .field("useBFramesAsRef", &self.0.useBFramesAsRef) // Enum
                .field("numRefL0", &self.0.numRefL0) // Enum
                .field("numRefL1", &self.0.numRefL1) // Enum
                .field("outputBitDepth", &self.0.outputBitDepth) // Enum
                .field("inputBitDepth", &self.0.inputBitDepth) // Enum
                .field("tfLevel", &self.0.tfLevel) // Enum
                .field("reserved1", &self.0.reserved1)
                .field("reserved2", &self.0.reserved2)
                .finish()
        }
    }

    pub struct HEVC(pub(crate) ffi::NV_ENC_CONFIG_HEVC);
    impl BaseNvCodec for HEVC {}
    impl Debug for HEVC {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("NV_ENC_CONFIG_HEVC")
                .field("level", &self.0.level)
                .field("tier", &self.0.tier)
                .field("minCUSize", &self.0.minCUSize) // Enum
                .field("maxCUSize", &self.0.maxCUSize) // Enum
                .field("idrPeriod", &self.0.idrPeriod)
                .field("intraRefreshPeriod", &self.0.intraRefreshPeriod)
                .field("intraRefreshCnt", &self.0.intraRefreshCnt)
                .field("maxNumRefFramesInDPB", &self.0.maxNumRefFramesInDPB)
                .field("ltrNumFrames", &self.0.ltrNumFrames)
                .field("vpsId", &self.0.vpsId)
                .field("spsId", &self.0.spsId)
                .field("ppsId", &self.0.ppsId)
                .field("sliceMode", &self.0.sliceMode)
                .field("sliceModeData", &self.0.sliceModeData)
                .field("maxTemporalLayersMinus1", &self.0.maxTemporalLayersMinus1)
                .field("hevcVUIParameters", &self.0.hevcVUIParameters) // Enum
                .field("ltrTrustMode", &self.0.ltrTrustMode)
                .field("useBFramesAsRef", &self.0.useBFramesAsRef) // Enum
                .field("numRefL0", &self.0.numRefL0) // Enum
                .field("numRefL1", &self.0.numRefL1) // Enum
                .field("tfLevel", &self.0.tfLevel) // Enum
                .field(
                    "disableDeblockingFilterIDC",
                    &self.0.disableDeblockingFilterIDC,
                )
                .field("outputBitDepth", &self.0.outputBitDepth) // Enum
                .field("inputBitDepth", &self.0.inputBitDepth) // Enum
                .field("numTemporalLayers", &self.0.numTemporalLayers)
                .field("numViews", &self.0.numViews)
                .field("reserved1", &self.0.reserved1)
                .field("reserved2", &self.0.reserved2)
                .finish()
        }
    }

    pub struct AV1(pub(crate) ffi::NV_ENC_CONFIG_AV1);
    impl BaseNvCodec for AV1 {}
    impl Debug for AV1 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // TODO: Convert fields with the 'Enum' comment to their string values
            f.debug_struct("NV_ENC_CONFIG_AV1")
                .field("level", &self.0.level)
                .field("tier", &self.0.tier)
                .field("minPartSize", &self.0.minPartSize) // Enum
                .field("maxPartSize", &self.0.maxPartSize) // Enum
                .field("idrPeriod", &self.0.idrPeriod)
                .field("intraRefreshPeriod", &self.0.intraRefreshPeriod)
                .field("intraRefreshCnt", &self.0.intraRefreshCnt)
                .field("maxNumRefFramesInDPB", &self.0.maxNumRefFramesInDPB)
                .field("numTileColumns", &self.0.numTileColumns)
                .field("numTileRows", &self.0.numTileRows)
                .field("reserved2", &self.0.reserved2)
                .field("tileWidths", &self.0.tileWidths)
                .field("tileHeights", &self.0.tileHeights)
                .field("maxTemporalLayersMinus1", &self.0.maxTemporalLayersMinus1)
                .field("colorPrimaries", &self.0.colorPrimaries) // Enum
                .field("transferCharacteristics", &self.0.transferCharacteristics) // Enum
                .field("matrixCoefficients", &self.0.matrixCoefficients) // Enum
                .field("colorRange", &self.0.colorRange)
                .field("useBFramesAsRef", &self.0.useBFramesAsRef) // Enum
                .field("filmGrainParams", &self.0.filmGrainParams) // Enum
                .field("numFwdRefs", &self.0.numFwdRefs) // Enum
                .field("numBwdRefs", &self.0.numBwdRefs) // Enum
                .field("outputBitDepth", &self.0.outputBitDepth) // Enum
                .field("inputBitDepth", &self.0.inputBitDepth) // Enum
                .field("ltrNumFrames", &self.0.ltrNumFrames)
                .field("numTemporalLayers", &self.0.numTemporalLayers)
                .field("tfLevel", &self.0.tfLevel) // Enum
                .field("reserved1", &self.0.reserved1)
                .field("reserved3", &self.0.reserved3)
                .finish()
        }
    }

    pub struct H264MeOnly(pub(crate) ffi::NV_ENC_CONFIG_H264_MEONLY);
    impl BaseNvCodec for H264MeOnly {}
    impl Debug for H264MeOnly {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("NV_ENC_CONFIG_H264_MEONLY")
                .field("reserved1", &self.0.reserved1)
                .field("reserved2", &self.0.reserved2)
                .finish()
        }
    }

    pub struct HEVCMeOnly(pub(crate) ffi::NV_ENC_CONFIG_HEVC_MEONLY);
    impl BaseNvCodec for HEVCMeOnly {}
    impl Debug for HEVCMeOnly {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("NV_ENC_CONFIG_HEVC_MEONLY")
                .field("reserved", &self.0.reserved)
                .field("reserved1", &self.0.reserved1)
                .finish()
        }
    }

    pub trait BaseNvCodec: Sized {
        fn new() -> Self {
            unsafe { std::mem::zeroed::<Self>() }
        }
    }
}
