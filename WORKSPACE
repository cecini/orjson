workspace(name = "orjson_repo")

# first self dep
# then dep's dep such pyo3'dep 
load("//:orjson_deps.bzl", "orjson_deps")
orjson_deps()


load("//:orjson_defs.bzl","setup_deps")
setup_deps()
