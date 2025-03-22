project(
    'gentoo-utils',
    'rust',
    meson_version: '>=1.6.1',
    default_options: {'rust_std': '2021'},
)

fs = import('fs')
rust = import('rust')

nom = dependency('nom-8-rs')

gentoo_utils = static_library('gentoo_utils', 'src/lib.rs', dependencies: [nom])

rust.test('gentoo_utils', gentoo_utils)
