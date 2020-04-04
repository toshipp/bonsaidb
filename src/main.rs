mod heap {
    use async_std::fs::File;
    use async_std::io::Error;
    use async_std::path::Path;
    use async_std::prelude::*;
    use crc::crc32::{Digest, Hasher32, IEEE};
    use std::convert::AsRef;
    use std::mem;
    use std::slice;

    struct Block {}

    struct Heap {
        file: File,
    }

    const PREAMBLE: [u8; 4] = [0, 1, 2, 3];
    const VERSION: u32 = 0;

    #[repr(C)]
    struct Header {
        preamble: [u8; 4],
        version: u32,
        checksum: u32,
    }

    impl Heap {
        pub async fn create(path: impl AsRef<Path>) -> Result<Heap, Error> {
            let mut file = File::create(path).await?;

            // write header
            let mut header = Header {
                preamble: PREAMBLE,
                version: VERSION,
                checksum: 0,
            };
            let header_slice =
                unsafe { slice::from_raw_parts::<u8>(&header as *const Header as *const u8, 16) };
            let mut hasher = Digest::new(IEEE);
            hasher.write(header_slice);
            header.checksum = hasher.sum32();
            let header_slice = unsafe {
                slice::from_raw_parts::<u8>(
                    &header as *const Header as *const u8,
                    mem::size_of::<Header>(),
                )
            };

            file.write_all(header_slice).await?;

            file.flush().await?;

            Ok(Heap { file: file })
        }
    }

    async fn open(path: impl AsRef<Path>) -> Result<Heap, ()> {
        todo!();
    }

    async fn allocate_block(heap: &mut Heap) -> Result<Block, ()> {
        todo!();
    }
}

fn main() {
    println!("Hello, world!");
}
