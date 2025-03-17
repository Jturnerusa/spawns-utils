use core::{assert_eq, convert::From};
use std::path::PathBuf;

use gentoo_utils::vdb::{Content, Dir, Obj, Sym};

fn test_contents() {
    let input = "obj /usr/share/alsa/ucm2/NXP/iMX8/Librem_5_Devkit/Librem 5 Devkit.conf 6c0d51586d94c272b160eb7ba6c61331 1739589188\ndir /a/path to something\nsym /a/path to something -> ../another path 102021\n";

    let expected = [
        Content::Obj(Obj {
            path: PathBuf::from(
                "/usr/share/alsa/ucm2/NXP/iMX8/Librem_5_Devkit/Librem 5 Devkit.conf",
            ),
            md5: String::from("6c0d51586d94c272b160eb7ba6c61331"),
            size: 1739589188,
        }),
        Content::Dir(Dir {
            path: PathBuf::from("/a/path to something"),
        }),
        Content::Sym(Sym {
            src: PathBuf::from("/a/path to something"),
            dest: PathBuf::from("../another path"),
            size: 102021,
        }),
    ];

    let contents = gentoo_utils::vdb::parsers::contents::parse_contents(input).unwrap();

    for (received, expected) in contents.iter().zip(expected) {
        assert_eq!(*received, expected);
    }
}

fn main() {
    test_contents();
}
