use std::fmt::Display;

pub mod nvenc_api;
pub mod nvenc_session;

pub struct NvEncApiError(u32);

impl From<u32> for NvEncApiError {
    fn from(value: u32) -> Self {
        NvEncApiError(value)
    }
}

impl Display for NvEncApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NVENCSTATUS({})", self.0)
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
