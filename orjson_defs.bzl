#load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")
load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")
#load("@io_bazel_rules_rust//:workspace.bzl", "rust_workspace")
load("@rules_pyo3_repo//cargo:crates.bzl", "rules_pyo3_fetch_remote_crates")
#load("@//cargo:crates.bzl", "raze_fetch_remote_crates")
#load("@orjson_repo//cargo:crates.bzl", orjson_fetch_remote_crates = "raze_fetch_remote_crates")
load("@orjson_repo//cargo:crates.bzl",  "raze_fetch_remote_crates")
#use the python.bzl in the rules_pyo3_repo
#load("@rules_pyo3_repo//:python.bzl", "setup_local_python")
#load("@toolchains//:python.bzl", "setup_local_python")
load("@rules_python//python:pip.bzl", "pip_install")
#load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
#load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository", "new_git_repository")

load("@toolchains//:toolchains_deps.bzl", toolchains_deps = "toolchains_deps")
load("@toolchains//:toolchains_defs.bzl", toolchains_setup_debugdeps = "setup_debugdeps", toolchains_setup_releasedeps = "setup_releasedeps")
def setup_deps():
    toolchains_deps()
    toolchains_setup_debugdeps()

 #   bazel_version(name = "bazel_version")

    #rust_workspace()

    rules_pyo3_fetch_remote_crates()

    raze_fetch_remote_crates()


    pip_install(   # or pip3_import
        name = "debug_deps",
	requirements = "@orjson_repo//:debug/requirements.txt",
        python_interpreter_target = "@python//:python",
        timeout = 600,
    )
# repo deps in repos.bzl, def in the defs.bzl 
