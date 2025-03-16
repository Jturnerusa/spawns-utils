use core::{assert_eq, convert::From};
use std::path::{Path, PathBuf};

use gentoo_utils::vdb;

fn contents() {
    let input = "obj /usr/share/alsa/ucm2/NXP/iMX8/Librem_5_Devkit/Librem 5 Devkit.conf 6c0d51586d94c272b160eb7ba6c61331 1739589188\ndir /a/path to something\nsym /a/path to something -> ../another path 102021\n";

    let expected = [
        vdb::Content::Obj(vdb::Obj {
            path: PathBuf::from(
                "/usr/share/alsa/ucm2/NXP/iMX8/Librem_5_Devkit/Librem 5 Devkit.conf",
            ),
            md5: String::from("6c0d51586d94c272b160eb7ba6c61331"),
            size: 1739589188,
        }),
        vdb::Content::Dir(vdb::Dir {
            path: PathBuf::from("/a/path to something"),
        }),
        vdb::Content::Sym(vdb::Sym {
            src: PathBuf::from("/a/path to something"),
            dest: PathBuf::from("../another path"),
            size: 102021,
        }),
    ];

    let contents = vdb::contents(input).unwrap();

    for (received, expected) in contents.iter().zip(expected) {
        assert_eq!(*received, expected);
    }
}

fn main() {
    contents();
}
