#[macro_use]
extern crate nom;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use nom::IResult;

pub fn open_file<T: AsRef<Path>>(path: T) -> Result<BufReader<File>, ::std::io::Error> {
    File::open(path).map(|f| BufReader::new(f))
}

enum MetadataBlockType {
    StreamInfo,
    Padding,
    Application,
    SeekTable,
    VorbisComment,
    CueSheet,
    Picture
}

struct MetadataBlockHeader {
    last_metadata_block: bool,
    block_type: MetadataBlockType,
    block_data_length: u32,
}

named!(metadata_block_header<&[u8], MetadataBlockHeader>,
    bits!(
        do_parse!(
            last_block: take_bits!(u8, 1) >>
            block_type: take_bits!(u8, 7) >>
            length: take_bits(u8, 24) >>
            (
                MetadataBlockHeader {
                    last_metadata_block: last_block,
                    block_type: match block_type {
                        0 => MetadataBlockType::StreamInfo,
                        1 => MetadataBlockType::Padding,
                        2 => MetadataBlockType::Application,
                        3 => MetadataBlockType::SeekTable,
                        4 => MetadataBlockType::VorbisComment,
                        5 => MetadataBlockType::CueSheet,
                        6 => MetadataBlockType::Picture,
                        _ => panic!(),
                    },
                    block_data_length: length
                }
            )
        )
    )
);

pub fn parse_flac_file_metadata<R: Read>(reader: R) -> Option<HashMap<String, String>> {
    unimplemented!()
}
