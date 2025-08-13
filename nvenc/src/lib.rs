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

impl NvEncInitializeParams {
    pub fn set_encode_guid(&mut self, encode_guid: ffi::GUID) -> &mut Self {
        self.0.encodeGUID = encode_guid;
        self
    }

    pub fn set_preset_guid(&mut self, preset_guid: ffi::GUID) -> &mut Self {
        self.0.presetGUID = preset_guid;
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
