#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]


extern crate libc;
pub mod src {
pub mod gifread {
pub mod gifread;
} // mod gifread
pub mod minitiff {
pub mod tiffread;
pub mod tiffutil;
pub mod tiffwrite;
} // mod minitiff
pub mod opngreduc {
pub mod opngreduc;
} // mod opngreduc
pub mod optipng {
pub mod bitset;
pub mod ioutil;
pub mod optim;
pub mod optipng;
pub mod ratio;
pub mod wildargs;
} // mod optipng
pub mod pngxtern {
pub mod pngxio;
pub mod pngxmem;
pub mod pngxrbmp;
pub mod pngxread;
pub mod pngxrgif;
pub mod pngxrjpg;
pub mod pngxrpnm;
pub mod pngxrtif;
pub mod pngxset;
} // mod pngxtern
pub mod pnmio {
pub mod pnmin;
pub mod pnmout;
pub mod pnmutil;
} // mod pnmio
} // mod src
