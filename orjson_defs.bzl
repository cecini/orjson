load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")
load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")
load("@io_bazel_rules_rust//:workspace.bzl", "rust_workspace")
load("@rules_pyo3_repo//cargo:crates.bzl", "rules_pyo3_fetch_remote_crates")
#load("@//cargo:crates.bzl", "raze_fetch_remote_crates")
#load("@orjson_repo//cargo:crates.bzl", orjson_fetch_remote_crates = "raze_fetch_remote_crates")
load("@orjson_repo//cargo:crates.bzl",  "raze_fetch_remote_crates")
#use the python.bzl in the rules_pyo3_repo
load("@rules_pyo3_repo//:python.bzl", "setup_local_python")
load("@rules_python//python:pip.bzl", "pip_install")
#load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
#load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository", "new_git_repository")
def setup_deps():
    # rust library so need rust
    rust_repositories(
            edition = "2018",
            version = "nightly",
 	    iso_date = "2020-11-25",
    )
    bazel_version(name = "bazel_version")

    rust_workspace()

    rules_pyo3_fetch_remote_crates()

    raze_fetch_remote_crates()

    # this repo should be in a external repo 
    # such as on anki or new repo 
    setup_local_python(name = "python")
    native.register_toolchains("@python//:python3_toolchain")

    pip_install(   # or pip3_import
        name = "debug_deps",
	requirements = "@orjson_repo//:debug/requirements.txt",
        python_interpreter_target = "@python//:python",
        timeout = 600,
    )
# repo deps in repos.bzl, def in the defs.bzl 
