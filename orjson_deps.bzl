"""Load dependencies needed to compile the orjson library as a 3rd-party consumer."""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository", "new_git_repository")
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")


def orjson_deps():
    """Loads common dependencies needed to compile the orjson library."""

    maybe(
        http_archive,
        name = "bazel_skylib",
        sha256 = "97e70364e9249702246c0e9444bccdc4b847bed1eb03c5a3ece4f83dfe6abc44",
        urls = [
            "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/1.0.2/bazel-skylib-1.0.2.tar.gz",
            "https://github.com/bazelbuild/bazel-skylib/releases/download/1.0.2/bazel-skylib-1.0.2.tar.gz",
        ],
    )
    
    maybe(
        git_repository,
        name = "io_bazel_rules_rust",
        commit = "18608544c831f6c999f216bb23ca52cf97c04480",
        remote = "https://github.com/ankitects/rules_rust",
        shallow_since = "1606951450 +1000",
    )

    maybe(
        git_repository,
        name = "rules_python",
        commit = "3927c9bce90f629eb5ab08bbc99a3d3bda1d95c0",
        remote = "https://github.com/ankitects/rules_python",
        shallow_since = "1604408056 +1000",
    )
    maybe(
        native.local_repository,
	name = "rules_pyo3_repo",
        path = "/workspaces/rules_pyo3",
    )
  #  maybe(
  #      git_repository,
  #      name = "rules_pyo3",
  #      #commit = "304d8974fa41e37e8ad3e32b9cb1221ecc9bb985",
  #	commit = "e05ab56eb313e75329c8461a1b9a4f772a0af739",
  #      remote = "https://github.com/cecini/rules_pyo3",
  #  )

    # have not set python toolchain,so no need put here 
    # pip_install have maybe 
    #pip_install(   # or pip3_import
    #    name = "debug_deps",
#	requirements = "@orjson_repo//:debug/requirements.txt",
#        python_interpreter_target = "@python//:python",
#        timeout = 600,
#    )

