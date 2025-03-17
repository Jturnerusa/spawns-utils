project(
    'gentoo-utils',
    'rust',
    meson_version: '>=1.6.1',
    default_options: {'rust_std': '2021'},
)

fs = import('fs')

nom = dependency('nom-8-rs')

sources = files('src/lib.rs', 'src/vdb/mod.rs')

gentoo_parsers = static_library('gentoo_utils', sources, dependencies: [nom])

tests = files('tests/vdb/parsers/contents.rs')

foreach t : tests
    exe_name = fs.stem(t)
    test_name = 'test' + '_' + exe_name
    test(test_name, executable(exe_name, [t], link_with: [gentoo_parsers]))
endforeach
