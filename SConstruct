import os
import sys

from SCons.Script import (
    ARGUMENTS,
    Copy,
    Default,
    Dir,
    Environment,
    Help,
    Mkdir,
)

Help("""
TikToken Lib
============

  scons                     Build static library + headers (default)
  scons shared=1            Build shared library (DLL/SO)
  scons test=1              Build and run GPT-2 compatibility test
  scons install             Copy artifacts to prefix=build/install

Cache (first run downloads vocab files):
  set TIKTOKEN_CACHE_DIR=C:\\path\\to\\cache
""")

root = Dir("#").abspath
build_dir = os.path.join(root, "build")
target_dir = os.path.join(root, "target", "release")
include_src = os.path.join(root, "include")
shared = ARGUMENTS.get("shared", "0") == "1"
run_tests = ARGUMENTS.get("test", "0") == "1"
prefix = ARGUMENTS.get("prefix", os.path.join(build_dir, "install"))

is_windows = sys.platform.startswith("win")
is_macos = sys.platform == "darwin"

if is_windows:
    static_name = "tiktoken.lib"
    shared_name = "tiktoken.dll"
    import_name = "tiktoken.dll.lib"
else:
    static_name = "libtiktoken.a"
    shared_name = "libtiktoken.so" if not is_macos else "libtiktoken.dylib"
    import_name = shared_name

env = Environment(ENV=os.environ.copy())

rust_sources = [
    "Cargo.toml",
    "Cargo.lock",
    os.path.join("src", "lib.rs"),
    os.path.join("src", "ffi.rs"),
    os.path.join("src", "load.rs"),
    os.path.join("src", "encoding.rs"),
]

cargo_stamp = env.Command(
    os.path.join(build_dir, "cargo.stamp"),
    rust_sources,
    "cargo build --release",
)

if shared:
    artifact = os.path.join(target_dir, shared_name)
else:
    artifact = os.path.join(target_dir, static_name)

built_lib = env.Command(
    os.path.join(build_dir, os.path.basename(artifact)),
    cargo_stamp,
    Copy("$TARGET", artifact),
)

built_header = env.Command(
    os.path.join(build_dir, "tiktoken.h"),
    os.path.join("include", "tiktoken.h"),
    Copy("$TARGET", "$SOURCE"),
)
built_hpp = env.Command(
    os.path.join(build_dir, "tiktoken.hpp"),
    os.path.join("include", "tiktoken.hpp"),
    Copy("$TARGET", "$SOURCE"),
)

cpp_env = env.Clone(CPPPATH=[include_src, build_dir], LIBPATH=[build_dir])

if is_windows:
    cpp_env.Append(CCFLAGS=["/W4", "/O2"])
    if shared:
        cpp_env.Append(LIBS=["tiktoken.dll"])
    else:
        cpp_env.Append(LIBS=["tiktoken", "ws2_32", "userenv", "bcrypt", "advapi32", "ntdll"])
else:
    cpp_env.Append(CCFLAGS=["-Wall", "-Wextra", "-O2"])
    if shared:
        cpp_env.Append(LIBS=["tiktoken"])
        cpp_env.Append(LINKFLAGS=[f"-Wl,-rpath,{build_dir}"])
    else:
        cpp_env.Append(LINKFLAGS=[os.path.join(build_dir, static_name)])

test_prog = cpp_env.Program(
    os.path.join(build_dir, "test_gpt2"),
    os.path.join("tests", "test_gpt2.c"),
)
env.Depends(test_prog, [built_lib, built_header])

if run_tests:
    passed = env.Command(
        os.path.join(build_dir, "test_gpt2.passed"),
        test_prog,
        "$SOURCE",
    )
    Default(passed)
else:
    Default([built_lib, built_header, built_hpp, test_prog])

install_inc = os.path.join(prefix, "include")
install_lib = os.path.join(prefix, "lib")
env.Alias("install", prefix)
env.Command(install_inc, None, MkDir(install_inc))
env.Command(install_lib, None, MkDir(install_lib))
env.Install(install_inc, [os.path.join("include", "tiktoken.h"), os.path.join("include", "tiktoken.hpp")])
env.Install(install_lib, built_lib)
if shared and is_windows:
    env.Install(install_lib, os.path.join(target_dir, import_name))
