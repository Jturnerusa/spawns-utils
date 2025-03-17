tests = files('vdb/parsers/contents.rs')

foreach t : tests
    exe_name = fs.stem(t)
    test_name = 'test' + '_' + exe_name
    test(test_name, executable(exe_name, [t], link_with: [gentoo_parsers]))
endforeach

