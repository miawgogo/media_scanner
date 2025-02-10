use std::io::Error as IOErr;


// A error type for the convert functions 
#[derive(Debug)]
pub enum ConvertErr{
    FfmpegErr,
    ProcError,
    IoErr(IOErr)
}

// Allows us to use ? for some calls we make to the IO crate
impl From<IOErr> for ConvertErr {
    fn from(value: IOErr) -> Self {
        ConvertErr::IoErr(value)
    }
}