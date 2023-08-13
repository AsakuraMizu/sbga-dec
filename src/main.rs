use std::{
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
};

use aes::cipher::{block_padding::NoPadding, BlockDecryptMut, KeyIvInit};
use clap::Parser;
use glob::glob;
use hex::FromHex;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, long)]
    key: String,
    #[arg(short, long, default_value_t = 0x200000)]
    offset: usize,
    inputs: Vec<String>,
}

type Decryptor = cbc::Decryptor<aes::Aes128Dec>;

type KeyIv = [u8; 0x10];

fn generate_iv(base_iv: &KeyIv, offset: usize) -> KeyIv {
    base_iv
        .iter()
        .enumerate()
        .map(|(i, x)| x ^ ((offset >> ((i % 8) << 3)) as u8) & 0xff)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn decrypt_chunk(chunk: &[u8], key: &KeyIv, iv: &KeyIv) -> Vec<u8> {
    let decryptor = Decryptor::new(key.into(), iv.into());
    decryptor
        .decrypt_padded_vec_mut::<NoPadding>(chunk)
        .expect("cannot decrypt")
}

fn main() {
    let cli = Cli::parse();

    let files = cli
        .inputs
        .iter()
        .flat_map(|input| glob(input).expect(&format!("cannot read glob pattern: {}", input)))
        .collect::<Result<Vec<_>, _>>()
        .expect("cannot list files");

    files.iter().for_each(|input| {
        println!("Decrypting {}...", input.display());

        let mut f = File::open(&input).expect("cannot open input file");
        f.seek(SeekFrom::Start(cli.offset as u64))
            .expect("cannot skip offset");

        let key: KeyIv = KeyIv::from_hex(&cli.key).expect("invalid key");
        let iv: KeyIv = [0u8; 0x10];

        let mut buf_header = [0u8; 0x10];
        f.read(&mut buf_header).expect("cannot read input file");
        let out_header = decrypt_chunk(&buf_header, &key, &iv);

        let header: &[u8; 0x10] = match input.extension().and_then(|x| x.to_str()) {
            Some("opt") => include_bytes!("exfat.bin"),
            Some("app") => include_bytes!("ntfs.bin"),
            _ => panic!("unexpected file exension"),
        };
        let iv: KeyIv = header
            .iter()
            .zip(out_header)
            .map(|(a, b)| a ^ b)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        f.seek(SeekFrom::Current(-0x10))
            .expect("cannot skip offset");
        let mut of = File::create(input.with_extension("vhd")).expect("cannot create out file");

        let mut buf = [0u8; 0x1000];
        let mut offset = 0;
        loop {
            match f.read(&mut buf).expect("cannot read input file") {
                0 => break,
                n => {
                    let iv: KeyIv = generate_iv(&iv, offset);
                    of.write_all(&decrypt_chunk(&buf, &key, &iv))
                        .expect("cannot write to output file");
                    offset += n;
                }
            }
        }
    });
}
