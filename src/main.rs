use std::{
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
    path::PathBuf,
};

use aes::cipher::{block_padding::NoPadding, BlockDecryptMut, KeyIvInit};
use anyhow::anyhow;
use clap::{Parser, ValueEnum};
use const_hex::{FromHex, FromHexError, ToHexExt};
use indicatif::{ProgressBar, ProgressStyle};

use crate::presets::Preset;

mod presets;

type Decryptor = cbc::Decryptor<aes::Aes128Dec>;

type KeyIv = [u8; 0x10];

fn key_parser(s: &str) -> Result<KeyIv, FromHexError> {
    KeyIv::from_hex(s)
}

#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    /// 16-digit hex string
    #[arg(short, long, group = "mode", requires = "fs", value_parser = key_parser)]
    key: Option<KeyIv>,

    /// Set fs type to exfat
    #[arg(long, requires = "key", group = "fs")]
    exfat: bool,

    /// Set fs type to ntfs
    #[arg(long, requires = "key", group = "fs")]
    ntfs: bool,

    /// Use a preset
    #[arg(short, long, group = "mode")]
    preset: Option<Preset>,

    /// Skip first n bytes
    #[arg(long, default_value_t = 0x200000)]
    offset: usize,

    /// File to decrypt
    file: PathBuf,

    /// Output path
    #[arg(short, long)]
    out_file: Option<PathBuf>,
}

fn generate_iv(base_iv: &KeyIv, offset: usize) -> KeyIv {
    base_iv
        .iter()
        .enumerate()
        .map(|(i, x)| x ^ ((offset >> ((i % 8) << 3)) as u8))
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

fn main() -> anyhow::Result<()> {
    let Cli {
        key,
        exfat,
        ntfs: _,
        preset,
        offset,
        file,
        out_file,
    } = Cli::parse();

    let preset = if let Some(preset) = preset {
        println!(
            "Using preset {:?}, key={}",
            preset,
            preset.key().encode_hex()
        );
        preset
    } else if let Some(key) = key {
        println!("Using custom key");
        Preset::Custom { key, exfat }
    } else if let Some(preset) = match file.extension().and_then(|x| x.to_str()) {
        Some("opt") => Some(Preset::OPT),
        Some("pack") => Some(Preset::PACK),
        Some("app") => file
            .file_name()
            .and_then(|s| s.to_str())
            .and_then(|s| Preset::from_str(&s[0..4], true).ok()),
        _ => None,
    } {
        println!(
            "Preset {:?} auto detected, key={}",
            preset,
            preset.key().encode_hex()
        );
        preset
    } else {
        return Err(anyhow!(
            "Unknown game / file extension, please specify a preset or key and file system type"
        ));
    };

    let key = preset.key();
    let header = preset.header();

    println!("Decrypting {}...", file.display());

    let mut f = File::open(&file)?;
    f.seek(SeekFrom::Start(offset as u64))?;

    let iv: KeyIv = [0u8; 0x10];

    let mut buf_header = [0u8; 0x10];
    f.read_exact(&mut buf_header)?;
    let out_header = decrypt_chunk(&buf_header, &key, &iv);

    let iv: KeyIv = header
        .iter()
        .zip(out_header)
        .map(|(a, b)| a ^ b)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let out_file = out_file.unwrap_or_else(|| file.with_extension("vhd"));
    let mut of = File::create(&out_file)?;
    f.seek(SeekFrom::Current(-0x10))?;

    let pb = ProgressBar::new(f.metadata().unwrap().len() - offset as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})",
        )
        .unwrap()
        .progress_chars("#>-"),
    );

    let mut buf = [0u8; 0x1000];
    let mut offset = 0;
    loop {
        match f.read(&mut buf)? {
            0 => break,
            n => {
                let iv: KeyIv = generate_iv(&iv, offset);
                of.write_all(&decrypt_chunk(&buf, &key, &iv))?;
                offset += n;
                pb.set_position(offset as u64);
            }
        }
    }

    pb.finish();
    println!("Decrypt finished, save to {}", out_file.display());
    Ok(())
}
