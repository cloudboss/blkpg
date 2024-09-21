# blkpg

A Rust library to call Linux blkpg ioctls.

Note: only the `BLKPG_RESIZE_PARTITION` operation is implemented.

## Example

```
use std::fs::File;
use std::io::Error;

use blkpg::resize_partition;

fn main() -> Result<(), Error> {
    let f = File::options()
        .read(true)
        .write(true)
        .open("/dev/nvme0n1")?;
    resize_partition(&f, 2, 456, 789, 512)?;
    Ok(())
}
```
