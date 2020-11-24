workspace(name = "orjson_repo")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository", "new_git_repository")


# Todo: abstarct this set for common use
# first self dep
# then dep's dep such pyo3'dep 
load("//:orjson_deps.bzl", "orjson_deps")
orjson_deps()

load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")
rust_repositories(
        edition = "2018",
        version = "nightly",
	iso_date = "2020-10-24",
)
load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")
bazel_version(name = "bazel_version")


load("@io_bazel_rules_rust//:workspace.bzl", "rust_workspace")
rust_workspace()


load("@rules_pyo3_repo//cargo:crates.bzl", "rules_pyo3_fetch_remote_crates")
rules_pyo3_fetch_remote_crates()


# Get local raze rust depend
load("@//cargo:crates.bzl", "raze_fetch_remote_crates")

raze_fetch_remote_crates()

# setup python3 toolchain 
load(":python.bzl", "setup_local_python")
setup_local_python(name = "python")
#native.register_toolchains("@python//:python3_toolchain")
register_toolchains("@python//:python3_toolchain")


load("@rules_python//python:pip.bzl", "pip_install")
pip_install(   # or pip3_import
    name = "debug_deps",
    requirements = "//:debug/requirements.txt",
    python_interpreter_target = "@python//:python",
    timeout = 600,
 )
