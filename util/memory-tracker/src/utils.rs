use std::fmt;

pub(crate) enum HumanReadableSize {
    Bytes(u64),
    KiBytes(f64),
    MiBytes(f64),
    GiBytes(f64),
}

impl fmt::Display for HumanReadableSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Bytes(v) => write!(f, "{} Bytes", v),
            Self::KiBytes(v) => write!(f, "{:.2} KiB", v),
            Self::MiBytes(v) => write!(f, "{:.2} MiB", v),
            Self::GiBytes(v) => write!(f, "{:.2} GiB", v),
        }
    }
}

impl From<u64> for HumanReadableSize {
    fn from(v: u64) -> Self {
        match v {
            _ if v < 1024 => Self::Bytes(v),
            _ if v < 1024 * 1024 => Self::KiBytes((v as f64) / 1024.0),
            _ if v < 1024 * 1024 * 1024 => Self::MiBytes((v as f64) / 1024.0 / 1024.0),
            _ => Self::GiBytes((v as f64) / 1024.0 / 1024.0 / 1024.0),
        }
    }
}
