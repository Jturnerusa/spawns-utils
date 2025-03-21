project(
    'gentoo-utils',
    'rust',
    meson_version: '>=1.6.1',
    default_options: {'rust_std': '2021'},
)

fs = import('fs')

nom = dependency('nom-8-rs')

static_library('gentoo_utils', 'src/lib.rs', dependencies: [nom])

subdir('tests')
