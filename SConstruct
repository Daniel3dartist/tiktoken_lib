import os
import sys

from SCons.Script import (
    ARGUMENTS,
    Copy,
    Default,
    Dir,
    Environment,
    Exit,
    Help,
    Mkdir,
)

Help("""
TikToken Lib
============

  scons                         Build static library + headers for the host OS (default)
  scons shared=1                Build shared library (DLL/SO) for the host OS
  scons test=1                  Build and run the GPT-2 compatibility test (host OS only)
  scons target=<rust-triple>    Cross-compile for another OS, e.g. target=x86_64-pc-windows-gnu
  scons install                 Copy artifacts to prefix=build/install

target= and shared= combine freely (static/dynamic x host/cross). Cross builds only compile
the C test binary — this host can't execute a foreign OS's binary, so test=1 is rejected
when combined with a target= that isn't natively runnable here.

Cache (first run downloads vocab files):
  set TIKTOKEN_CACHE_DIR=C:\\path\\to\\cache
""")

root = Dir("#").abspath
build_dir = os.path.join(root, "build")
include_src = os.path.join(root, "include")
shared = ARGUMENTS.get("shared", "0") == "1"
run_tests = ARGUMENTS.get("test", "0") == "1"
prefix = ARGUMENTS.get("prefix", os.path.join(build_dir, "install"))
rust_target = ARGUMENTS.get("target")

host_is_windows = sys.platform.startswith("win")
host_is_macos = sys.platform == "darwin"

if rust_target:
    if "windows-msvc" in rust_target:
        target_os = "windows-msvc"
    elif "windows-gnu" in rust_target:
        target_os = "windows-gnu"
    elif "apple-darwin" in rust_target:
        target_os = "macos"
    else:
        target_os = "linux"
elif host_is_windows:
    target_os = "windows-msvc"
elif host_is_macos:
    target_os = "macos"
else:
    target_os = "linux"

can_execute_here = (
    (target_os in ("windows-msvc", "windows-gnu") and host_is_windows)
    or (target_os == "macos" and host_is_macos)
    or (target_os == "linux" and not host_is_windows and not host_is_macos)
)

if target_os == "windows-msvc" and not host_is_windows:
    print(
        "error: target resolves to the MSVC ABI, which needs Microsoft's own linker and "
        "can't be cross-compiled from this host. Use target=x86_64-pc-windows-gnu to "
        "cross-compile a Windows build from here, or build natively on Windows for MSVC.",
        file=sys.stderr,
    )
    Exit(1)

if target_os == "macos" and not host_is_macos:
    print(
        "error: cross-compiling to macOS isn't supported by this build; build natively on "
        "macOS instead.",
        file=sys.stderr,
    )
    Exit(1)

if run_tests and not can_execute_here:
    print(
        f"error: test=1 needs to run the built binary, which this host can't execute for "
        f"target_os={target_os!r}. Drop test=1 for cross builds — the binary is still built "
        "and can be copied to a real machine of that OS to run by hand.",
        file=sys.stderr,
    )
    Exit(1)

target_dir = (
    os.path.join(root, "target", rust_target, "release")
    if rust_target
    else os.path.join(root, "target", "release")
)

if target_os == "windows-msvc":
    static_name = "tiktoken.lib"
    shared_name = "tiktoken.dll"
    import_name = "tiktoken.dll.lib"
elif target_os == "windows-gnu":
    static_name = "libtiktoken.a"
    shared_name = "tiktoken.dll"
    import_name = "libtiktoken.dll.a"
elif target_os == "macos":
    static_name = "libtiktoken.a"
    shared_name = "libtiktoken.dylib"
    import_name = shared_name
else:
    static_name = "libtiktoken.a"
    shared_name = "libtiktoken.so"
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

cargo_cmd = "cargo build --release"
if rust_target:
    cargo_cmd += f" --target {rust_target}"

cargo_stamp = env.Command(
    os.path.join(build_dir, "cargo.stamp"),
    rust_sources,
    cargo_cmd,
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

# Linking against a dynamic lib needs the import library alongside it in build_dir;
# Cargo only ever writes it next to the .dll/.so in target_dir.
built_import_lib = None
if shared and target_os in ("windows-msvc", "windows-gnu"):
    built_import_lib = env.Command(
        os.path.join(build_dir, os.path.basename(import_name)),
        cargo_stamp,
        Copy("$TARGET", os.path.join(target_dir, import_name)),
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

if target_os == "windows-msvc":
    cpp_env.Append(CCFLAGS=["/W4", "/O2"])
    if shared:
        cpp_env.Append(LIBS=["tiktoken.dll"])
    else:
        cpp_env.Append(LIBS=["tiktoken", "ws2_32", "userenv", "bcrypt", "advapi32", "ntdll"])
elif target_os == "windows-gnu":
    mingw_cc = "gcc" if host_is_windows else "x86_64-w64-mingw32-gcc"
    cpp_env["CC"] = mingw_cc
    cpp_env["LINK"] = mingw_cc
    cpp_env["PROGSUFFIX"] = ".exe"
    cpp_env.Append(CCFLAGS=["-Wall", "-Wextra", "-O2"])
    if shared:
        cpp_env.Append(LIBS=["tiktoken"])
    else:
        cpp_env.Append(LIBS=["tiktoken", "ws2_32", "userenv", "bcrypt", "advapi32", "ntdll"])
else:
    cpp_env.Append(CCFLAGS=["-Wall", "-Wextra", "-O2"])
    if shared:
        cpp_env.Append(LIBS=["tiktoken"])
        cpp_env.Append(LINKFLAGS=[f"-Wl,-rpath,{build_dir}"])
    else:
        # --whole-archive avoids link-order issues: a plain LINKFLAGS path places the
        # archive before the object files on the command line, and ld only resolves
        # symbols from an archive against references seen *before* it.
        cpp_env.Append(
            LINKFLAGS=[
                "-Wl,--whole-archive",
                os.path.join(build_dir, static_name),
                "-Wl,--no-whole-archive",
            ]
        )
        cpp_env.Append(LIBS=["pthread", "dl", "m"])

test_prog = cpp_env.Program(
    os.path.join(build_dir, "test_gpt2"),
    os.path.join("tests", "test_gpt2.c"),
)
test_prog_deps = [built_lib, built_header]
if built_import_lib is not None:
    test_prog_deps.append(built_import_lib)
env.Depends(test_prog, test_prog_deps)

if run_tests:
    passed = env.Command(
        os.path.join(build_dir, "test_gpt2.passed"),
        test_prog,
        # A bare "$SOURCE" (nothing else in the string) is a known SCons foot-gun: it
        # resolves to the raw substituted value instead of a command string, so the
        # action silently no-ops instead of running the test binary. Quoting forces
        # proper string substitution.
        '"$SOURCE"',
    )
    Default(passed)
else:
    Default([built_lib, built_header, built_hpp, test_prog])

install_inc = os.path.join(prefix, "include")
install_lib = os.path.join(prefix, "lib")
env.Alias("install", prefix)
env.Command(install_inc, None, Mkdir(install_inc))
env.Command(install_lib, None, Mkdir(install_lib))
env.Install(install_inc, [os.path.join("include", "tiktoken.h"), os.path.join("include", "tiktoken.hpp")])
env.Install(install_lib, built_lib)
if built_import_lib is not None:
    env.Install(install_lib, built_import_lib)
