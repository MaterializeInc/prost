pub mod search {
    include!(concat!(env!("OUT_DIR"), "/search.rs"));
}
#[allow(dead_code)]
pub const FILE_DESCRIPTOR_SET_DATA: &[u8] = include_bytes!("file_descriptor_set.pb");
