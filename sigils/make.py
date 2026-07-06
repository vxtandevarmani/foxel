#!/usr/bin/env python3

import pathlib
import argparse
import subprocess
import sys

PROJECT_NAME  = "sigils"
COMPILER      = "clang"
LINKER_PATH   = "linker.ld"
CPP_VERSION   = "-std=c++20"

RISCV_FLAGS   = "--target=riscv64-unknown-elf -march=rv64im -mcmodel=medany -fuse-ld=lld -flto" #-flto
X64_FLAGS     = "--target=x86_64-w64-mingw32 -mno-sse -masm=intel -Wl,--no-seh,--enable-stdcall-fixup"

CFLAGS = (
    f"-Os -nostdlib -fno-asynchronous-unwind-tables {CPP_VERSION} "
    "-fno-ident -fpack-struct=8 -falign-functions=1 -s -w "
    "-ffunction-sections -falign-jumps=1 -falign-labels=1 "
    "-Wl,-s -fno-exceptions -fno-builtin "
    "-fms-extensions -fPIC -Iinclude -Wl,-Tscripts/linker.ld"
)

ELF_DATA = "*(.rodata*)"
PE_DATA  = "*(.rdata*)"

LINKER_SCRIPT = """
ENTRY(_start)
SECTIONS
{
    .text :
    {
        *(.text*)
        __my_boundry = .;
        {REPLACED}
        LONG(__my_boundry);
    }
}
"""

def run_cmd(cmd):
    print("[*] Running:", " ".join(cmd))
    try:
        subprocess.run(cmd, check=True)
    except subprocess.CalledProcessError as e:
        print(f"[!] Command failed with exit code {e.returncode}")
        sys.exit(1)

def arginit():
    parser = argparse.ArgumentParser()
    parser.add_argument("--action", required=True, help="clean, risc, x64")
    parser.add_argument("--debug", action="store_true")
    return parser.parse_args()

def clean():
    bin_dir = pathlib.Path("bin")
    obj_dir = bin_dir / "obj"

    obj_dir.mkdir(parents=True, exist_ok=True)

    for path in bin_dir.rglob("*"):
        if path.is_file():
            path.unlink()

    print("[+] Cleaned bin directory")

def patch_linker_script(arch):
    linker_path = pathlib.Path("scripts") / LINKER_PATH

    data = ELF_DATA if arch == "risc" else PE_DATA
    linker_path.write_text(
        LINKER_SCRIPT.replace("{REPLACED}", data),
        encoding="utf-8"
    )

    print("[+] Linker script patched")


def compile_objects(flags):
    src_files = list(pathlib.Path("src").rglob("*.cpp"))
    obj_dir = pathlib.Path("bin/obj")
    obj_dir.mkdir(parents=True, exist_ok=True)

    obj_files = []

    for src in src_files:
        obj = obj_dir / (src.stem + ".o")
        obj_files.append(str(obj))

        cmd = [COMPILER, "-c", str(src), "-o", str(obj)]
        cmd += flags.split()

        run_cmd(cmd)

    return obj_files


def link_binary(obj_files, flags, output):
    cmd = [COMPILER] + obj_files + ["-o", output]
    cmd += flags.split()
    run_cmd(cmd)


def extract_shellcode(name: str, arch: str):

    bin_dir = pathlib.Path("bin")

    if arch == "risc":
        ext = "elf"
    elif arch == "x64":
        ext = "exe"
    else:
        print(f"[!] Unknown architecture: {arch}")
        sys.exit(1)

    input_file = bin_dir / f"{name}.{arch}.{ext}"
    output_file = bin_dir / f"{name}.{arch}.bin"

    if not input_file.exists():
        print(f"[!] Input file not found: {input_file}")
        sys.exit(1)

    cmd = [
        "llvm-objcopy",
        "--dump-section",
        f".text={output_file}",
        str(input_file)
    ]

    print(f"[*] Extracting shellcode for {arch}...")
    print("[*] Running:", " ".join(cmd))

    try:
        subprocess.run(cmd, check=True)
    except subprocess.CalledProcessError as e:
        print(f"[!] llvm-objcopy failed with exit code {e.returncode}")
        sys.exit(1)
    except FileNotFoundError:
        print("[!] llvm-objcopy not found in PATH")
        sys.exit(1)

    size = output_file.stat().st_size
    print(f"[+] Extracted {size} bytes -> {output_file}")


def generate_risc():
    patch_linker_script("risc")

    total_flags = RISCV_FLAGS + " " + CFLAGS

    objs = compile_objects(total_flags)
    link_binary(objs, total_flags, f"bin/{PROJECT_NAME}.risc.elf")
    extract_shellcode(PROJECT_NAME, "risc")

    print("[+] RISC build complete")


def generate_x64():
    patch_linker_script("x64")

    total_flags = X64_FLAGS + " " + CFLAGS

    objs = compile_objects(total_flags)
    link_binary(objs, total_flags, f"bin/{PROJECT_NAME}.x64.exe")
    extract_shellcode(PROJECT_NAME, "x64")

    print("[+] x64 build complete")


def main():
    tree = {
        "clean": [clean],
        "risc":  [clean, generate_risc],
        "x64":   [clean, generate_x64],
    }

    args = arginit()

    action = args.action.lower()
    if action not in tree:
        print("Invalid action")
        return

    for step in tree[action]:
        step()


if __name__ == "__main__":
    main()
