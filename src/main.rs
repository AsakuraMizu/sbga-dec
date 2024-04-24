use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    path::PathBuf,
    sync::mpsc::channel,
    thread::spawn,
};

use cbc::cipher;
use cipher::{inout::InOutBuf, BlockDecryptMut, KeyIvInit};
use clap::{Parser, ValueEnum};
use const_hex::{FromHex, FromHexError, ToHexExt};
use indicatif::{ProgressBar, ProgressStyle};

use crate::presets::Preset;

mod presets;

type KeyOrIv = [u8; 0x10];

fn key_parser(s: &str) -> Result<KeyOrIv, FromHexError> {
    KeyOrIv::from_hex(s)
}

#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    /// 16-digit hex string
    #[arg(short, long, group = "mode", requires = "fs", value_parser = key_parser)]
    key: Option<KeyOrIv>,

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
    offset: u64,

    /// File to decrypt
    file: PathBuf,

    /// Output path
    #[arg(short, long)]
    out_file: Option<PathBuf>,
}

fn decrypt_chunk_mut(buf: &mut [u8], key: &KeyOrIv, iv: &KeyOrIv) {
    let mut decryptor = cbc::Decryptor::<aes::Aes128Dec>::new(key.into(), iv.into());
    let (blocks, _) = InOutBuf::from(buf).into_chunks();
    decryptor.decrypt_blocks_inout_mut(blocks);
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
        return Err(anyhow::anyhow!(
            "Unknown game / file extension, please specify a preset or key and file system type"
        ));
    };

    let key = preset.key();
    let header = preset.header();

    println!("Decrypting {}...", file.display());

    let mut f = File::open(&file)?;
    f.seek(SeekFrom::Start(offset))?;

    let iv: KeyOrIv = [0u8; 0x10];

    let mut buf_header = [0u8; 0x10];
    f.read_exact(&mut buf_header)?;
    decrypt_chunk_mut(&mut buf_header, &key, &iv);
    f.seek(SeekFrom::Current(-0x10))?;

    let iv: KeyOrIv = header
        .iter()
        .zip(buf_header)
        .map(|(a, b)| a ^ b)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    println!("Calculated iv={}", iv.encode_hex());

    let out_file = out_file.unwrap_or_else(|| file.with_extension("vhd"));
    let of = File::create(&out_file)?;

    let pb = ProgressBar::new(f.metadata().unwrap().len() - offset).with_style(
        ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} {binary_bytes_per_sec} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    let (tx, rx) = channel::<[u8; 0x1000]>();
    let handle = spawn(move || -> anyhow::Result<()> {
        let mut writer = BufWriter::new(of);
        for buf in rx {
            writer.write_all(&buf)?;
        }
        Ok(())
    });

    let mut reader = BufReader::new(f);
    let mut buf = [0u8; 0x1000];
    let mut local_iv = [0u8; 0x10];
    let mut offset = 0;
    loop {
        match reader.read(&mut buf)? {
            0 => break,
            n => {
                for i in 0..16 {
                    local_iv[i] = iv[i] ^ ((offset >> ((i % 8) << 3)) as u8);
                }
                decrypt_chunk_mut(&mut buf, &key, &local_iv);
                tx.send(buf)?;
                offset += n;
                pb.set_position(offset as u64);
            }
        }
    }

    drop(tx);
    handle.join().unwrap()?;
    pb.finish();
    println!("Decrypt finished, save to {}", out_file.display());
    Ok(())
}
