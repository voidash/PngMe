// TODO implement CRC 

//table carrying all 8 bit messages
static mut table: [u64;256];
static mut is_table_computed:bool = false;


fn make_crc_table() {
    let c:u32;
    // let (n,k):(i32,i32);
    for n in 0..256 {
        c = n;
        for k in 0..8 {
            if c & 1 {
                c = 0xedb88320L ^ (c >> 1);
                // 1110 1101 1011 1000 1000 0011 0010 0000
            }else {
                c = c >> 1;
            }
        }
    }


}
