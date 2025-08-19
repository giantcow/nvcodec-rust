use std::{error, fmt};

use nvenc_sys::ffi;

#[derive(Debug, Clone, PartialEq)]
pub struct UnknownGuid(pub ffi::GUID, &'static str);

impl fmt::Display for UnknownGuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UnknownGuid({:?}) {}", self.0, self.1)
    }
}

impl error::Error for UnknownGuid {}

pub mod codecs {
    use super::*;

    pub enum Codec {
        AVC,
        HEVC,
        AV1,
    }

    impl From<self::Codec> for ffi::GUID {
        fn from(value: self::Codec) -> Self {
            match value {
                Codec::AVC => ffi::NV_ENC_CODEC_H264_GUID,
                Codec::HEVC => ffi::NV_ENC_CODEC_HEVC_GUID,
                Codec::AV1 => ffi::NV_ENC_CODEC_AV1_GUID,
            }
        }
    }

    impl TryFrom<ffi::GUID> for self::Codec {
        type Error = UnknownGuid;

        fn try_from(value: ffi::GUID) -> Result<Self, Self::Error> {
            if value == ffi::NV_ENC_CODEC_H264_GUID {
                Ok(Self::AVC)
            } else if value == ffi::NV_ENC_CODEC_HEVC_GUID {
                Ok(Self::HEVC)
            } else if value == ffi::NV_ENC_CODEC_AV1_GUID {
                Ok(Self::AV1)
            } else {
                Err(UnknownGuid(
                    value,
                    "Invalid codec GUID. Must be one of AVC, HEVC, or AV1",
                ))
            }
        }
    }
}

pub mod profiles {
    use super::*;

    pub trait ProfileLike
    where
        Self: Into<ffi::GUID> + TryFrom<ffi::GUID> + PartialEq,
    {
        #[allow(clippy::wrong_self_convention)] // TODO: Add lifetimes
        fn as_raw(self) -> ffi::GUID {
            self.into()
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum AVCProfile {
        AutoSelect = 0,
        Baseline,
        Main,
        High,
        High10,
        High422,
        High444,
        ProgressiveHigh,
        ConstrainedHigh,
        Stereo,
    }

    impl ProfileLike for AVCProfile {}

    impl From<AVCProfile> for ffi::GUID {
        fn from(value: AVCProfile) -> Self {
            match value {
                AVCProfile::AutoSelect => ffi::NV_ENC_CODEC_PROFILE_AUTOSELECT_GUID,
                AVCProfile::Baseline => ffi::NV_ENC_H264_PROFILE_BASELINE_GUID,
                AVCProfile::Main => ffi::NV_ENC_H264_PROFILE_MAIN_GUID,
                AVCProfile::High => ffi::NV_ENC_H264_PROFILE_HIGH_GUID,
                AVCProfile::High10 => ffi::NV_ENC_H264_PROFILE_HIGH_10_GUID,
                AVCProfile::High422 => ffi::NV_ENC_H264_PROFILE_HIGH_422_GUID,
                AVCProfile::High444 => ffi::NV_ENC_H264_PROFILE_HIGH_444_GUID,
                AVCProfile::ProgressiveHigh => ffi::NV_ENC_H264_PROFILE_PROGRESSIVE_HIGH_GUID,
                AVCProfile::ConstrainedHigh => ffi::NV_ENC_H264_PROFILE_CONSTRAINED_HIGH_GUID,
                AVCProfile::Stereo => ffi::NV_ENC_H264_PROFILE_STEREO_GUID,
            }
        }
    }

    impl TryFrom<ffi::GUID> for self::AVCProfile {
        type Error = UnknownGuid;

        fn try_from(value: ffi::GUID) -> Result<Self, Self::Error> {
            if value == ffi::NV_ENC_CODEC_PROFILE_AUTOSELECT_GUID {
                Ok(Self::AutoSelect)
            } else if value == ffi::NV_ENC_H264_PROFILE_BASELINE_GUID {
                Ok(Self::Baseline)
            } else if value == ffi::NV_ENC_H264_PROFILE_MAIN_GUID {
                Ok(Self::Main)
            } else if value == ffi::NV_ENC_H264_PROFILE_HIGH_GUID {
                Ok(Self::High)
            } else if value == ffi::NV_ENC_H264_PROFILE_HIGH_10_GUID {
                Ok(Self::High10)
            } else if value == ffi::NV_ENC_H264_PROFILE_HIGH_422_GUID {
                Ok(Self::High422)
            } else if value == ffi::NV_ENC_H264_PROFILE_HIGH_444_GUID {
                Ok(Self::High444)
            } else if value == ffi::NV_ENC_H264_PROFILE_PROGRESSIVE_HIGH_GUID {
                Ok(Self::ProgressiveHigh)
            } else if value == ffi::NV_ENC_H264_PROFILE_CONSTRAINED_HIGH_GUID {
                Ok(Self::ConstrainedHigh)
            } else if value == ffi::NV_ENC_H264_PROFILE_STEREO_GUID {
                Ok(Self::Stereo)
            } else {
                Err(UnknownGuid(value, "Known values: AutoSelect/Baseline/Main/High/High10/High422/High444/ProgressiveHigh/ConstrainedHigh/Stereo"))
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum HEVCProfile {
        AutoSelect = 0,
        FREXT,
        Main,
        Main10,
    }

    impl ProfileLike for HEVCProfile {}

    impl From<HEVCProfile> for ffi::GUID {
        fn from(value: HEVCProfile) -> Self {
            match value {
                HEVCProfile::AutoSelect => ffi::NV_ENC_CODEC_PROFILE_AUTOSELECT_GUID,
                HEVCProfile::Main => ffi::NV_ENC_HEVC_PROFILE_MAIN_GUID,
                HEVCProfile::Main10 => ffi::NV_ENC_HEVC_PROFILE_MAIN10_GUID,
                HEVCProfile::FREXT => ffi::NV_ENC_HEVC_PROFILE_FREXT_GUID,
            }
        }
    }

    impl TryFrom<ffi::GUID> for self::HEVCProfile {
        type Error = UnknownGuid;

        fn try_from(value: ffi::GUID) -> Result<Self, Self::Error> {
            if value == ffi::NV_ENC_CODEC_PROFILE_AUTOSELECT_GUID {
                Ok(Self::AutoSelect)
            } else if value == ffi::NV_ENC_HEVC_PROFILE_FREXT_GUID {
                Ok(Self::FREXT)
            } else if value == ffi::NV_ENC_HEVC_PROFILE_MAIN_GUID {
                Ok(Self::Main)
            } else if value == ffi::NV_ENC_HEVC_PROFILE_MAIN10_GUID {
                Ok(Self::Main10)
            } else {
                Err(UnknownGuid(
                    value,
                    "Known values: AutoSelect/FREXT/Main/Main10",
                ))
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum AV1Profile {
        AutoSelect = 0,
        Main,
    }

    impl ProfileLike for AV1Profile {}

    impl From<AV1Profile> for ffi::GUID {
        fn from(value: AV1Profile) -> Self {
            match value {
                AV1Profile::AutoSelect => ffi::NV_ENC_CODEC_PROFILE_AUTOSELECT_GUID,
                AV1Profile::Main => ffi::NV_ENC_AV1_PROFILE_MAIN_GUID,
            }
        }
    }

    impl TryFrom<ffi::GUID> for self::AV1Profile {
        type Error = UnknownGuid;

        fn try_from(value: ffi::GUID) -> Result<Self, Self::Error> {
            if value == ffi::NV_ENC_CODEC_PROFILE_AUTOSELECT_GUID {
                Ok(Self::AutoSelect)
            } else if value == ffi::NV_ENC_AV1_PROFILE_MAIN_GUID {
                Ok(Self::Main)
            } else {
                Err(UnknownGuid(value, "Known values: AutoSelect/Main"))
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        const FAKE_GUID: ffi::GUID = ffi::GUID {
            Data1: 0x0,
            Data2: 0x0,
            Data3: 0x0,
            Data4: [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
        };

        // ==================================================
        //                      AVC
        // ==================================================

        #[test]
        fn avc_can_get_value() {
            assert_eq!(
                AVCProfile::Baseline.as_raw(),
                ffi::NV_ENC_H264_PROFILE_BASELINE_GUID
            );
        }

        #[test]
        fn avc_can_try_from_ffi_guid() {
            assert_eq!(
                AVCProfile::Baseline,
                AVCProfile::try_from(ffi::NV_ENC_H264_PROFILE_BASELINE_GUID).unwrap(),
            )
        }

        #[test]
        fn avc_returns_error_with_invalid_ffi_guid() {
            assert_eq!(
                Some(UnknownGuid(FAKE_GUID, "Known values: AutoSelect/Baseline/Main/High/High10/High422/High444/ProgressiveHigh/ConstrainedHigh/Stereo")),
                AVCProfile::try_from(FAKE_GUID).err()
            );
        }

        // ==================================================
        //                      HEVC
        // ==================================================

        #[test]
        fn hevc_can_get_value() {
            assert_eq!(
                HEVCProfile::Main.as_raw(),
                ffi::NV_ENC_HEVC_PROFILE_MAIN_GUID
            );
        }

        #[test]
        fn hevc_can_try_from_ffi_guid() {
            assert_eq!(
                HEVCProfile::Main,
                HEVCProfile::try_from(ffi::NV_ENC_HEVC_PROFILE_MAIN_GUID).unwrap()
            );
        }

        #[test]
        fn hevc_returns_error_with_invalid_ffi_guid() {
            assert_eq!(
                Some(UnknownGuid(
                    FAKE_GUID,
                    "Known values: AutoSelect/FREXT/Main/Main10"
                )),
                HEVCProfile::try_from(FAKE_GUID).err()
            )
        }

        // ==================================================
        //                      AV1
        // ==================================================

        #[test]
        fn av1_can_get_value() {
            assert_eq!(AV1Profile::Main.as_raw(), ffi::NV_ENC_AV1_PROFILE_MAIN_GUID);
        }

        #[test]
        fn av1_can_try_from_ffi_guid() {
            assert_eq!(
                AV1Profile::Main,
                AV1Profile::try_from(ffi::NV_ENC_AV1_PROFILE_MAIN_GUID).unwrap()
            );
        }

        #[test]
        fn av1_returns_error_with_invalid_ffi_guid() {
            assert_eq!(
                Some(UnknownGuid(FAKE_GUID, "Known values: AutoSelect/Main")),
                AV1Profile::try_from(FAKE_GUID).err()
            )
        }
    }
}

pub mod presets {
    use super::*;

    #[derive(Debug, PartialEq)]
    pub enum Preset {
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
    }

    impl From<self::Preset> for ffi::GUID {
        fn from(value: self::Preset) -> Self {
            match value {
                Preset::P1 => ffi::NV_ENC_PRESET_P1_GUID,
                Preset::P2 => ffi::NV_ENC_PRESET_P2_GUID,
                Preset::P3 => ffi::NV_ENC_PRESET_P3_GUID,
                Preset::P4 => ffi::NV_ENC_PRESET_P4_GUID,
                Preset::P5 => ffi::NV_ENC_PRESET_P5_GUID,
                Preset::P6 => ffi::NV_ENC_PRESET_P6_GUID,
                Preset::P7 => ffi::NV_ENC_PRESET_P7_GUID,
            }
        }
    }

    impl TryFrom<ffi::GUID> for self::Preset {
        type Error = UnknownGuid;

        fn try_from(value: ffi::GUID) -> Result<Self, Self::Error> {
            if value == ffi::NV_ENC_PRESET_P1_GUID {
                Ok(Self::P1)
            } else if value == ffi::NV_ENC_PRESET_P2_GUID {
                Ok(Self::P2)
            } else if value == ffi::NV_ENC_PRESET_P3_GUID {
                Ok(Self::P3)
            } else if value == ffi::NV_ENC_PRESET_P4_GUID {
                Ok(Self::P4)
            } else if value == ffi::NV_ENC_PRESET_P5_GUID {
                Ok(Self::P5)
            } else if value == ffi::NV_ENC_PRESET_P6_GUID {
                Ok(Self::P6)
            } else if value == ffi::NV_ENC_PRESET_P7_GUID {
                Ok(Self::P7)
            } else {
                Err(UnknownGuid(value, "Known values: P{1..7}"))
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn can_get_value() {
            assert_eq!(
                Preset::P1,
                Preset::try_from(ffi::NV_ENC_PRESET_P1_GUID).unwrap()
            );
        }

        #[test]
        fn try_from_fake_guid_returns_error() {
            let fake_guid = ffi::GUID {
                Data1: 0x0,
                Data2: 0x0,
                Data3: 0x0,
                Data4: [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
            };

            assert_eq!(
                Some(UnknownGuid(fake_guid, "Known values: P{1..7}")),
                Preset::try_from(fake_guid).err()
            );
        }
    }
}
