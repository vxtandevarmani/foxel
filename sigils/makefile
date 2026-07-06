NAME   := sigils

CCX64  := clang
LDX64  := /mingw64/bin/ld

#CFLAGS := --target=riscv64-unknown-elf -march=rv64im -mcmodel=medany -fuse-ld=lld -flto
CFLAGS := --target=x86_64-w64-mingw32 -mno-sse -masm=intel

CFLAGS += -Os -nostdlib -fno-asynchronous-unwind-tables -std=c++20 
CFLAGS += -fno-ident -fpack-struct=8 -falign-functions=1 -w
CFLAGS += -ffunction-sections -falign-jumps=1 -falign-labels=1
CFLAGS += -fno-exceptions -fno-builtin
CFLAGS += -fms-extensions -fPIC -Iincludes
CFLAGS += -c

LDFLAGS := -m i386pep -s --no-seh --enable-stdcall-fixup
LDFLAGS += -T scripts/linker.ld

SRC    := $(wildcard src/*.cpp)
OBJ64  := $(SRC:%.cpp=%.x64.obj)

release: x64
debug:   x64-debug

x64-debug: CFLAGS += -D DEBUG
x64-debug: x64

ensure-dirs:
	@ mkdir -p bin/obj

x64: ensure-dirs $(OBJ64)
	@ echo "compiling x64 project"
	@ $(LDX64) $(LDFLAGS) -o bin/$(NAME).x64.exe bin/obj/*.x64.obj
	@ llvm-objcopy --dump-section .text=bin/$(NAME).x64.bin bin/$(NAME).x64.exe
#	@ rm bin/$(NAME).x64.exe

%.x64.obj: %.cpp
	@ echo "-> compiling $< to $(notdir $@)"
	@ $(CCX64) -o bin/obj/$(notdir $@) $< $(CFLAGS)

stomper:
	@ $(CCX64) test/stomper.cpp -o test/stomper.x64.exe -w

clean:
	@ rm -f bin/obj/*.x64.obj
	@ rm -f bin/*.exe
	@ rm -f bin/*.bin
	@ echo "removed object files"