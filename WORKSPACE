load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository", "new_git_repository")
# Todo: abstarct this set for common use
# first self dep
# then dep's dep such pyo3'dep 
################
# Python Rules #
################

git_repository(
    name = "rules_python",
    commit = "3927c9bce90f629eb5ab08bbc99a3d3bda1d95c0",
    remote = "https://github.com/ankitects/rules_python",
    shallow_since = "1604408056 +1000",
)

##############
# Rust Rules #
##############

git_repository(
    name = "io_bazel_rules_rust",
    commit = "a364ded42d9788144cd26b6e98d6b4038753bfa9",
    remote = "https://github.com/ankitects/rules_rust",
    shallow_since = "1604550071 +1000",
)

http_archive(
    name = "bazel_skylib",
    sha256 = "97e70364e9249702246c0e9444bccdc4b847bed1eb03c5a3ece4f83dfe6abc44",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/1.0.2/bazel-skylib-1.0.2.tar.gz",
        "https://github.com/bazelbuild/bazel-skylib/releases/download/1.0.2/bazel-skylib-1.0.2.tar.gz",
    ],
)

load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")

#rust_repositories(edition = "2018")
rust_repositories(
        edition = "2018",
        # use_worker = True,
        #version = "1.47.0",
        version = "nightly",
	iso_date = "2020-10-24",
    )

load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")

bazel_version(name = "bazel_version")
#bazel_version(name = "io_bazel_rules_rust_bazel_version")

# setup python3 toolchain 

##############
# PyO3 Rules #
##############

git_repository(
    name = "rules_pyo3",
    commit = "304d8974fa41e37e8ad3e32b9cb1221ecc9bb985",
    remote = "https://github.com/cecini/rules_pyo3",
    shallow_since = "1605383653 +0000",
)

load("@io_bazel_rules_rust//:workspace.bzl", "rust_workspace")

rust_workspace()

load("@rules_pyo3//cargo:crates.bzl", "rules_pyo3_fetch_remote_crates")

rules_pyo3_fetch_remote_crates()


# local raze rust depend
load("@//cargo:crates.bzl", "raze_fetch_remote_crates")
raze_fetch_remote_crates()
