"""Install the Debian MinGW cross tools in /workspace with pinned SHA-256 checks."""

import hashlib
import subprocess
from pathlib import Path

BASE = "https://deb.debian.org/debian/"
CACHE = Path("/workspace/.cache/debian")
DEST = Path("/workspace/.toolchains/mingw")

# Debian 13 amd64 package filenames and SHA-256 values from its package index.
PACKAGES = {
    "binutils-mingw-w64-x86-64": ("pool/main/b/binutils-mingw-w64/binutils-mingw-w64-x86-64_2.44-3+12+b1_amd64.deb", "b53f4a091b16a0d0952b8e3bd6027931641386258f96b37db1f0c62c787221d3"),
    "gcc-mingw-w64-base": ("pool/main/g/gcc-mingw-w64/gcc-mingw-w64-base_14.2.0-19+27+b1_amd64.deb", "8d2c64b886ab4a435a78ddc4cd3b510f149b65fbd6be8472e8f460627562718a"),
    "gcc-mingw-w64-x86-64-posix": ("pool/main/g/gcc-mingw-w64/gcc-mingw-w64-x86-64-posix_14.2.0-19+27+b1_amd64.deb", "e32d24cfd1482394f16d9a84c1d0a9f5aa08ee826a79acc383c70cc8c23c4fe5"),
    "gcc-mingw-w64-x86-64-posix-runtime": ("pool/main/g/gcc-mingw-w64/gcc-mingw-w64-x86-64-posix-runtime_14.2.0-19+27+b1_amd64.deb", "c898d96177574a8a7238564a1e799d8ddfed5a3d2ea005510305f9ad466a6f59"),
    "mingw-w64-common": ("pool/main/m/mingw-w64/mingw-w64-common_12.0.0-5_all.deb", "97dce5d0d8aff1cade4786ad4ef5078345477a598829afcac2e7f2fbc5399990"),
    "mingw-w64-x86-64-dev": ("pool/main/m/mingw-w64/mingw-w64-x86-64-dev_12.0.0-5_all.deb", "0bf89cf7454cccb49cd2a46a6f7b33896f6ae3a1a2fe33e02c40be6155bbb385"),
}

CACHE.mkdir(parents=True, exist_ok=True)
DEST.mkdir(parents=True, exist_ok=True)
for name, (path, expected) in PACKAGES.items():
    archive = CACHE / f"{name}.deb"
    if not archive.exists() or hashlib.sha256(archive.read_bytes()).hexdigest() != expected:
        subprocess.run(
            ["curl", "--fail", "--silent", "--show-error", "--location", "--retry", "3", BASE + path, "-o", str(archive)],
            check=True,
        )
    if hashlib.sha256(archive.read_bytes()).hexdigest() != expected:
        raise RuntimeError(f"SHA-256 mismatch: {name}")
    subprocess.run(["dpkg-deb", "-x", str(archive), str(DEST)], check=True)

gcc = DEST / "usr/bin/x86_64-w64-mingw32-gcc"
if not gcc.exists():
    gcc.symlink_to("x86_64-w64-mingw32-gcc-posix")
print("MinGW cross tools ready and package hashes verified")
