mod gifread {
    pub mod gifread;
}
mod minitiff {
    pub mod tiffread;
    pub mod tiffutil;
    pub mod tiffwrite;
}
mod opngreduc {

    pub mod opngreduc;
}
mod optipng {
    pub mod bitset;
    pub mod ioutil;
    pub mod optim;
    pub mod optipng;
    pub mod ratio;
    pub mod wildargs;
}
mod pngxtern {
    pub mod pngxio;
    pub mod pngxmem;
    pub mod pngxrbmp;
    pub mod pngxread;
    pub mod pngxrgif;
    pub mod pngxrjpg;
    pub mod pngxrpnm;
    pub mod pngxrtif;
    pub mod pngxset;
}
mod pnmio {
    pub mod pnmin;
    pub mod pnmout;
    pub mod pnmutil;
}

fn main() {
    println!("Running optipng Clippy analysis...");
}
