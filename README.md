# sbga-dec

Advanced sega hdd decryptor, written in rust

## Features

- Auto detect game & file type from **filename** and select key
- Auto calculate iv (_no iv option actually_)

## Usage

```
Advanced sega hdd decryptor, written in rust

Usage: sbga-dec [OPTIONS] <FILE>

Arguments:
  <FILE>  File to decrypt

Options:
  -k, --key <KEY>            16-digit hex string
      --exfat                Set fs type to exfat
      --ntfs                 Set fs type to ntfs
  -p, --preset <PRESET>      Use a preset [possible values: opt, pack, sbzs, sbzt, sbzu, sbzv, sdap, sdaq, sdav, sdbe, sdbn, sdbt, sdbx, sdbz, sdca, sdcd, sdcf, sdch, sdcr, sdct, sdcx, sddb, sddd, sddf, sddj, sddl, sddm, sddn, sddp, sdds, sddt, sddu, sddw, sddx, sdea, sdeb, sdec, sded, sdee, sdeg, sdej, sdem, sdep, sder, sdet, sdeu, sdev, sdez, sdfa, sdfe, sdfg, sdfl, sdfn, sdfp, sdft, sdfv, sdga, sdgb, sdgh, sdgk, sdgp, sdgq, sdgs, sdgt, sdgv, sdgy, sdgz, sdhd, sdhh, sdhj, sdhk, sdhn, sdhr]
      --offset <OFFSET>      Skip first n bytes [default: 2097152]
  -o, --out-file <OUT_FILE>  Output path
  -h, --help                 Print help
  -V, --version              Print version
```

## Examples

```bash
# The following commands are equivalent
./sbga-dec SDDT_1.45.00_20240214110456_0.app
./sbga-dec SDDT_1.45.00_20240214110456_0.app -o SDDT_1.45.00_20240214110456_0.vhd
./sbga-dec -p SDDT SDDT_1.45.00_20240214110456_0.app
./sbga-dec -k 3f7658728b9517d3314e684fa2e2a045 --ntfs SDDT_1.45.00_20240214110456_0.app
```
