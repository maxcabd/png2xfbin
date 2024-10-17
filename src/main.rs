
use std::path::Path;

use xfbin::{nucc_chunk::*, write_xfbin, NuccBinary, NuccStructInfo};
use xfbin::{xfbin::XfbinPage, Xfbin};




fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: png2xfbin <png file>");
        return;
    }

    let xfbin = png2xfbin(std::path::Path::new(&args[1]));

    // Save xfbin to path of png file
    let xfbin_path = std::path::Path::new(&args[1]).with_extension("xfbin");

    let _ = write_xfbin(xfbin, &xfbin_path);
}


/// Converts png file to xfbin file
pub fn png2xfbin(filepath: &Path) -> Xfbin {
    let png_bytes: Vec<u8> = std::fs::read(filepath).unwrap();

    let mut xfbin = Xfbin::default();
    let mut page = XfbinPage::default();

    let struct_info = NuccStructInfo {
        chunk_name: filepath.file_stem().unwrap().to_str().unwrap().to_string(),
        chunk_type: NuccChunkType::NuccChunkBinary.to_string(),
        filepath: filepath.file_name().unwrap().to_str().unwrap().to_string(),
    };

    // Png binary nucc struct
    let nucc_binary = Box::new(NuccBinary {
        struct_info,
        version: 121,
        data: png_bytes,
    });

    page.structs.push(nucc_binary);

    xfbin.pages.push(page);

    xfbin
}
