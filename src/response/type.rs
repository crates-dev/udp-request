use crate::*;

pub type BoxResponseTrait =
    Box<dyn ResponseTrait<OutputText = UdpResponseText, OutputBinary = UdpResponseBinary>>;
