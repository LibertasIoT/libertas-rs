use libertas_macros::LibertasExport;

#[derive(LibertasExport)]
struct Media {
    #[libertas_mime_types("image/png,image/jpeg")]
    photo: Vec<u8>,
    raw: Vec<u8>,
}

#[test]
fn mime_types_is_parser_metadata_and_raw_bytes_remain_available() {
    let media = Media { photo: vec![1, 2], raw: vec![0] };
    assert_eq!(media.photo, [1, 2]);
    assert_eq!(media.raw, [0]);
}
