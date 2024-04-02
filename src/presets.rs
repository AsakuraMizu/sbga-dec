use clap::ValueEnum;
use hex_literal::hex;

use crate::KeyIv;

macro_rules! generate_presets {
    ( $( $id:ident $key:literal )* ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
        #[allow(clippy::upper_case_acronyms)]
        pub enum Preset {
            $($id,)*
            #[value(skip)]
            Custom { key: KeyIv, exfat: bool },
        }
        impl Preset {
            pub fn key(&self) -> KeyIv {
                match self {
                    $(Self::$id => hex!($key),)*
                    Self::Custom { key, .. } => *key,
                }
            }
        }
    };
}

generate_presets! {
    OPT "5c84 a9e7 26ea a5dd 351f 2b07 50c2 3697"
    PACK "e428 1bcf 48c4 d28e b057 72ce 6f98 587a"
    SBZS "2ecb cff6 5ce0 abec c105 47f8 ac83 51d8"
    SBZT "9ab9 ce55 ed9c 194a 715a 73a7 699f 795b"
    SBZU "eb12 2825 4cdd 3077 eb3e 441c 0227 bf40"
    SBZV "3274 a399 594d 8477 9625 940b 69c0 2d3f"
    SDAP "41b5 027c 5e99 d94a a933 5d6d 7183 8ecf"
    SDAQ "c28f 22bc 1b33 9ae6 4180 7398 86dc 83d6"
    SDAV "eed9 5513 266a 499a 55e2 65b0 4916 9c44"
    SDBE "7053 fb94 4572 e5b6 31a6 65ce f4b5 bcdd"
    SDBN "c1f1 4ae2 e85b 095e 313c 8bae c125 805e"
    SDBT "a6a8 7067 1fd4 32ec 637a df7a 822f 97da"
    SDBX "3dc1 9c2d 0c20 ac19 9d5f a46e 7f63 35a6"
    SDBZ "521b de44 60f4 184e dd87 9136 adee a5ee"
    SDCA "1649 490a 03d6 c2ae c1c4 9698 2cb0 405c"
    SDCD "43b3 8502 d8f6 d3c7 b02b 95fc 28db 5308"
    SDCF "df98 6883 da83 7538 e37b 959a 3e41 17cd"
    SDCH "e2da 769e 94f1 d3ac a193 0cdb e070 8c9f"
    SDCR "4961 a51f d36f 14e7 2664 f523 7305 2160"
    SDCT "d6ae 51f1 0ec7 6da9 3c98 1800 fc3a d3cb"
    SDCX "7950 4ccc 509b 67d1 f7a3 f593 e6f9 d9d6"
    SDDB "8756 79b2 cd16 3796 2b0d b25c 51fb 21a6"
    SDDD "564e 9678 73de 6cbc d22e feca 6952 e9dc"
    SDDF "6505 8573 a0cb 8174 9e69 4ae1 64c6 1b04"
    SDDJ "630f e522 7653 7bd7 fb26 7adf 175f 4e99"
    SDDL "9924 5829 5fd0 6d6a 8af0 dfb3 f685 4c19"
    SDDM "0127 9582 10f6 ae9b deb8 9750 18b5 af24"
    SDDN "41dd 8e66 2901 17ac 67d3 11a2 f0a6 416e"
    SDDP "cf6d 6442 7eec a476 74e1 7bcd 46d1 ea8c"
    SDDS "161b ec6d 9098 9d0e 26d7 9117 0607 a440"
    SDDT "3f76 5872 8b95 17d3 314e 684f a2e2 a045"
    SDDU "649a e998 2625 f90c 55af 8671 3c55 d3fd"
    SDDW "1185 65d3 44f3 e14c a692 99ee ac04 9bb9"
    SDDX "428b ff0f 9e7a afc1 69a7 a757 51ff da98"
    SDEA "9f9c f148 ac3c 50aa f925 af1d fb27 f58b"
    SDEB "d511 ed69 0415 f635 9843 a134 fd47 836a"
    SDEC "f272 e501 6863 af2b a033 7f50 de68 6f6e"
    SDED "21fc ec77 9a16 769f 5277 a36f b542 992c"
    SDEE "191e b744 0672 dab0 8ddb b719 5efb 356f"
    SDEG "7218 53db e2d3 0baf e24f 0edb d210 deeb"
    SDEJ "9de1 ea6a e38d 9011 f55d 8ee8 6439 5d24"
    SDEM "7006 17f2 9369 6c07 fb9f 356d 3b99 240d"
    SDEP "fa2b 7ca5 3a82 3c15 2d94 0972 cbf5 32f5"
    SDER "7d73 367e bb21 8ec8 2930 d58d c6d7 950b"
    SDET "4643 e7b2 c300 6e02 6416 3edc 8545 fb72"
    SDEU "23b3 e9bb 47e3 ac99 98f6 e6c1 adc4 ae33"
    SDEV "3c1f 018d 8892 6d98 163b 07a1 563a 4818"
    SDEZ "d136 eba0 5d40 e826 82e6 aad8 d9e8 688c"
    SDFA "8e81 6b43 62db 24a2 3087 7885 864d 206d"
    SDFE "f617 19c3 71e5 bca6 788c 139a 5309 1617"
    SDFG "3398 fb86 bfe6 30a1 4979 4118 7986 1ac7"
    SDFL "2449 b480 67b9 176a 6e0f 9563 481e 97f4"
    SDFN "29f6 2e22 c6a9 fd8b e327 631c 6854 6405"
    SDFP "570b 8726 3a7c a0aa 4c13 88e2 04ee 6d4b"
    SDFT "92a2 5f38 8c50 737e 39c3 c2f0 0664 5f31"
    SDFV "fe82 db9a 6029 5d82 9b95 f03c 2276 018b"
    SDGA "0a66 10a6 2ef6 70c6 5b7e 7b17 50ff b7a1"
    SDGB "7ca4 e6b6 f3d6 e8b2 6472 9738 87d7 fa3a"
    SDGH "b3e3 0e7e abac 3767 ade1 3c69 c9b2 f22b"
    SDGK "9dc4 a17f c39f ca5a 8a35 8984 801c aaa7"
    SDGP "c87a b312 47e7 b6ff 95fd d79f b91f 9f37"
    SDGQ "c535 6dae 7b06 6bce 8898 4aec 36de b62d"
    SDGS "a515 0cc5 065d 2c59 ee2f 8f33 2cbd 29d5"
    SDGT "9d0b ba20 d1e8 4f24 5939 9f53 83be ee72"
    SDGV "573f 8cc4 4f10 f31e c749 b695 ebe8 86a8"
    SDGY "c04b 663a 5905 5acb dfeb c6d3 df0e 6a04"
    SDGZ "9ad7 4efb 208d 6ee4 fe5e e770 3317 12cf"
    SDHD "3abd 00d7 a820 ce86 2eaf 474b f6c8 f33e"
    SDHH "fc6f 887f 3717 c5d6 7131 13b9 2fa3 fb27"
    SDHJ "985e a66e cb5b 1f20 8c90 e2b8 98f0 b073"
    SDHK "bc92 d63c 2a09 9ca2 315a 483c 3041 fdd7"
    SDHN "8921 23a2 6d7c 03d4 9edd 12a8 0ee0 c58f"
    SDHR "1fb8 97ca b97c 8170 a6ac 0a21 685c 58d9"
}

impl Preset {
    pub fn header(&self) -> [u8; 0x10] {
        match self {
            Self::OPT | Self::Custom { exfat: true, .. } => {
                hex!("eb76 9045 5846 4154 2020 2000 0000 0000")
            }
            _ => hex!("eb52 904e 5446 5320 2020 2000 1001 0000"),
        }
    }
}
