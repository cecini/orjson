"""
@generated
cargo-raze generated Bazel file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""

load("@bazel_tools//tools/build_defs/repo:git.bzl", "new_git_repository")  # buildifier: disable=load
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")  # buildifier: disable=load
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")  # buildifier: disable=load

def raze_fetch_remote_crates():
    """This function defines a collection of repos and should be called in a WORKSPACE file"""
    maybe(
        http_archive,
        name = "raze__arrayvec__0_5_2",
        url = "https://crates.io/api/v1/crates/arrayvec/0.5.2/download",
        type = "tar.gz",
        strip_prefix = "arrayvec-0.5.2",
        build_file = Label("//cargo/remote:BUILD.arrayvec-0.5.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__associative_cache__1_0_1",
        url = "https://crates.io/api/v1/crates/associative-cache/1.0.1/download",
        type = "tar.gz",
        strip_prefix = "associative-cache-1.0.1",
        build_file = Label("//cargo/remote:BUILD.associative-cache-1.0.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__bitflags__1_2_1",
        url = "https://crates.io/api/v1/crates/bitflags/1.2.1/download",
        type = "tar.gz",
        strip_prefix = "bitflags-1.2.1",
        build_file = Label("//cargo/remote:BUILD.bitflags-1.2.1.bazel"),
    )

    maybe(
        new_git_repository,
        name = "raze__bytecount__0_6_0",
        remote = "https://github.com/ijl/orjson.git",
        commit = "417e40e6fcc3d5203f7e0824074b57a5c6497a49",
        build_file = Label("//cargo/remote:BUILD.bytecount-0.6.0.bazel"),
        init_submodules = True,
    )

    maybe(
        http_archive,
        name = "raze__cfg_if__0_1_10",
        url = "https://crates.io/api/v1/crates/cfg-if/0.1.10/download",
        type = "tar.gz",
        strip_prefix = "cfg-if-0.1.10",
        build_file = Label("//cargo/remote:BUILD.cfg-if-0.1.10.bazel"),
    )

    maybe(
        new_git_repository,
        name = "raze__encoding_rs__0_8_24",
        remote = "https://github.com/ijl/orjson.git",
        commit = "417e40e6fcc3d5203f7e0824074b57a5c6497a49",
        build_file = Label("//cargo/remote:BUILD.encoding_rs-0.8.24.bazel"),
        init_submodules = True,
    )

    maybe(
        http_archive,
        name = "raze__getrandom__0_1_15",
        url = "https://crates.io/api/v1/crates/getrandom/0.1.15/download",
        type = "tar.gz",
        strip_prefix = "getrandom-0.1.15",
        build_file = Label("//cargo/remote:BUILD.getrandom-0.1.15.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__inlinable_string__0_1_13",
        url = "https://crates.io/api/v1/crates/inlinable_string/0.1.13/download",
        type = "tar.gz",
        strip_prefix = "inlinable_string-0.1.13",
        build_file = Label("//cargo/remote:BUILD.inlinable_string-0.1.13.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__itoa__0_4_6",
        url = "https://crates.io/api/v1/crates/itoa/0.4.6/download",
        type = "tar.gz",
        strip_prefix = "itoa-0.4.6",
        build_file = Label("//cargo/remote:BUILD.itoa-0.4.6.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__lexical_core__0_7_4",
        url = "https://crates.io/api/v1/crates/lexical-core/0.7.4/download",
        type = "tar.gz",
        strip_prefix = "lexical-core-0.7.4",
        build_file = Label("//cargo/remote:BUILD.lexical-core-0.7.4.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__libc__0_2_80",
        url = "https://crates.io/api/v1/crates/libc/0.2.80/download",
        type = "tar.gz",
        strip_prefix = "libc-0.2.80",
        build_file = Label("//cargo/remote:BUILD.libc-0.2.80.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__libm__0_1_4",
        url = "https://crates.io/api/v1/crates/libm/0.1.4/download",
        type = "tar.gz",
        strip_prefix = "libm-0.1.4",
        build_file = Label("//cargo/remote:BUILD.libm-0.1.4.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__once_cell__1_5_2",
        url = "https://crates.io/api/v1/crates/once_cell/1.5.2/download",
        type = "tar.gz",
        strip_prefix = "once_cell-1.5.2",
        build_file = Label("//cargo/remote:BUILD.once_cell-1.5.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__packed_simd_2__0_3_4",
        url = "https://crates.io/api/v1/crates/packed_simd_2/0.3.4/download",
        type = "tar.gz",
        strip_prefix = "packed_simd_2-0.3.4",
        build_file = Label("//cargo/remote:BUILD.packed_simd_2-0.3.4.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__proc_macro2__1_0_24",
        url = "https://crates.io/api/v1/crates/proc-macro2/1.0.24/download",
        type = "tar.gz",
        strip_prefix = "proc-macro2-1.0.24",
        build_file = Label("//cargo/remote:BUILD.proc-macro2-1.0.24.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__quote__1_0_7",
        url = "https://crates.io/api/v1/crates/quote/1.0.7/download",
        type = "tar.gz",
        strip_prefix = "quote-1.0.7",
        build_file = Label("//cargo/remote:BUILD.quote-1.0.7.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__rand_core__0_5_1",
        url = "https://crates.io/api/v1/crates/rand_core/0.5.1/download",
        type = "tar.gz",
        strip_prefix = "rand_core-0.5.1",
        build_file = Label("//cargo/remote:BUILD.rand_core-0.5.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__ryu__1_0_5",
        url = "https://crates.io/api/v1/crates/ryu/1.0.5/download",
        type = "tar.gz",
        strip_prefix = "ryu-1.0.5",
        build_file = Label("//cargo/remote:BUILD.ryu-1.0.5.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__serde__1_0_117",
        url = "https://crates.io/api/v1/crates/serde/1.0.117/download",
        type = "tar.gz",
        strip_prefix = "serde-1.0.117",
        build_file = Label("//cargo/remote:BUILD.serde-1.0.117.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__serde_derive__1_0_117",
        url = "https://crates.io/api/v1/crates/serde_derive/1.0.117/download",
        type = "tar.gz",
        strip_prefix = "serde_derive-1.0.117",
        build_file = Label("//cargo/remote:BUILD.serde_derive-1.0.117.bazel"),
    )

    maybe(
        new_git_repository,
        name = "raze__serde_json__1_0_53",
        remote = "https://github.com/ijl/orjson.git",
        commit = "417e40e6fcc3d5203f7e0824074b57a5c6497a49",
        build_file = Label("//cargo/remote:BUILD.serde_json-1.0.53.bazel"),
        init_submodules = True,
    )

    maybe(
        http_archive,
        name = "raze__smallvec__1_5_0",
        url = "https://crates.io/api/v1/crates/smallvec/1.5.0/download",
        type = "tar.gz",
        strip_prefix = "smallvec-1.5.0",
        build_file = Label("//cargo/remote:BUILD.smallvec-1.5.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__static_assertions__1_1_0",
        url = "https://crates.io/api/v1/crates/static_assertions/1.1.0/download",
        type = "tar.gz",
        strip_prefix = "static_assertions-1.1.0",
        build_file = Label("//cargo/remote:BUILD.static_assertions-1.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__syn__1_0_51",
        url = "https://crates.io/api/v1/crates/syn/1.0.51/download",
        type = "tar.gz",
        strip_prefix = "syn-1.0.51",
        build_file = Label("//cargo/remote:BUILD.syn-1.0.51.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__unicode_xid__0_2_1",
        url = "https://crates.io/api/v1/crates/unicode-xid/0.2.1/download",
        type = "tar.gz",
        strip_prefix = "unicode-xid-0.2.1",
        build_file = Label("//cargo/remote:BUILD.unicode-xid-0.2.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wasi__0_9_0_wasi_snapshot_preview1",
        url = "https://crates.io/api/v1/crates/wasi/0.9.0+wasi-snapshot-preview1/download",
        type = "tar.gz",
        strip_prefix = "wasi-0.9.0+wasi-snapshot-preview1",
        build_file = Label("//cargo/remote:BUILD.wasi-0.9.0+wasi-snapshot-preview1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wyhash__0_4_2",
        url = "https://crates.io/api/v1/crates/wyhash/0.4.2/download",
        type = "tar.gz",
        strip_prefix = "wyhash-0.4.2",
        build_file = Label("//cargo/remote:BUILD.wyhash-0.4.2.bazel"),
    )
