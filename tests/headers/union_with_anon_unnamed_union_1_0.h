// bindgen-flags: --rust-target 1.0 --with-derive-hash --with-derive-partialeq

union foo {
    unsigned int a;
    union {
        unsigned short b;
        unsigned char c;
    };
};
