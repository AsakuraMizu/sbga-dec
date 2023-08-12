use std::{
    fs::{self, File},
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
};

use aes::cipher::{block_padding::NoPadding, BlockDecryptMut, KeyIvInit};
use clap::Parser;
use hex::FromHex;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, long)]
    key: String,
    #[arg(short, long, default_value_t = 0x200000)]
    offset: u64,
    inputs: Vec<PathBuf>,
}

type Decryptor = cbc::Decryptor<aes::Aes128Dec>;

type KeyIv = [u8; 0x10];

fn generate_iv(base_iv: &KeyIv, offset: u64) -> KeyIv {
    base_iv
        .iter()
        .enumerate()
        .map(|(i, x)| x ^ ((offset >> ((i % 8) << 3)) as u8) & 0xff)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn decrypt(data: &Vec<u8>, key: &KeyIv, iv: &KeyIv) -> Vec<u8> {
    let mut result = Vec::<u8>::new();
    data.chunks(0x1000).enumerate().for_each(|(i, d)| {
        let iv = generate_iv(iv, i as u64 * 0x1000);
        let decryptor = Decryptor::new(key.into(), &iv.into());
        let mut out = decryptor
            .decrypt_padded_vec_mut::<NoPadding>(d)
            .expect("cannot decrypt");
        result.append(&mut out);
    });
    result
}

fn main() {
    let cli = Cli::parse();

    cli.inputs.iter().for_each(|input| {
        println!("Decrypting {}...", input.display());
        let mut f = File::open(&input).expect("cannot open input file");
        f.seek(SeekFrom::Start(cli.offset))
            .expect("cannot skip offset");
        let mut buf = Vec::<u8>::new();
        f.read_to_end(&mut buf).expect("cannot read input file");

        let key: KeyIv = KeyIv::from_hex(&cli.key).expect("invalid key");

        let iv: KeyIv = [0u8; 0x10];
        let out = decrypt(&buf, &key, &iv);

        let header: &[u8; 0x10] = match input.extension().and_then(|x| x.to_str()) {
            Some("opt") => include_bytes!("exfat.bin"),
            Some("app") => include_bytes!("ntfs.bin"),
            _ => panic!("unexpected file exension"),
        };
        let out_header: [u8; 0x10] = out
            .iter()
            .take(0x10)
            .cloned()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let iv: KeyIv = header
            .iter()
            .zip(out_header)
            .map(|(a, b)| a ^ b)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let out = decrypt(&buf, &key, &iv);
        fs::write(input.with_extension("vhd"), &out).expect("cannot write to out file");
    });
}
