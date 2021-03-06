# implementation of PngMe Project 

[PickleNerd](https://picklenerd.github.io/pngme_book/introduction.html)

- Contains implementation of PNG Specification
- Can encode and Decode secret message to PNG file. 

```

USAGE:
    pngme [OPTIONS] --mode <MODE> --file <FILE> --chunk <CHUNK>

OPTIONS:
    -c, --chunk <CHUNK>        4 character String, 3rd character must be capital
    -f, --file <FILE>          png file path
    -h, --help                 Print help information
    -m, --message <MESSAGE>    message to hide
        --mode <MODE>          mode: encode or decode
    -V, --version              Print version information

```


## Building

```
$ cargo build
```

### From PNG specification

- *Length* : 4 byte unsigned integer, thta gives the number of bytes in data field
- *chunk_type* : 4 byte chunk type code. consists usually ASICC A-Z and a-z letters 'ArYa' is a valid chunk_type
- *chunk_data* : according to chunk type , it is the data bytes
- *CRC* : Cyclic Redundancy Check 


#### Chunk Type (AaKb)
Chunk Type is 4 byte array. 
 - First byte has ancillary bit : if that chunk is necessary or not (uppercase) = necessary , 
 - Second byte has private bit: if uppercase public and is necessary 
 - Third bytes has Reserved bit : if uppercase means its This version of PNG
 - Safe-to-Copy bit : if uppercase can't copy by photo editors , if lowercase can be copied


## TODO
- [x] use clap to process command line
- add link image support
- finish implementing crc