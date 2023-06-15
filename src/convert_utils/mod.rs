/*
Enthält die Funktionalität Binären Code in Gensequenzen oder Gensequenzen in Binären Code zu übersetzen.
*/

mod encoder;
mod decoder;
mod error_correcting;

pub use encoder::easy_encode;

pub use decoder::easy_decode;

