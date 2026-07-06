#pragma once

#include "defs.h"
#include "utils.h"

struct supercall {
    uintptr_t call_id;
    uintptr_t args[13];    
};

struct functions {
    struct {
        uintptr_t handle;
        struct {
            D_API(LoadLibraryA)
            D_API(GetProcAddress)
        };
    } kernel32 = {
        RESOLVE_TYPE(LoadLibraryA),
        RESOLVE_TYPE(GetProcAddress)
    };

    struct {
        uintptr_t handle;
        struct {
            D_API(MessageBoxA)
        };
    } user32 = {
        RESOLVE_TYPE(MessageBoxA)
    };

};

enum SSN {
    HostCall    = 0xff,
    GetPeb      = 0x45,
    Invalid     = 0x00,
};


#if defined(__riscv) && __riscv_xlen == 64
    
    inline uintptr_t host_call(uintptr_t address, uintptr_t* args) {
        register uintptr_t a0 asm("a0") = address;
        register uintptr_t a1 asm("a1") = (uintptr_t)args;
        register uintptr_t a7 asm("a7") = HostCall;
        
        asm volatile ("scall" : "+r"(a0) : "r"(a1), "r"(a7) );
        
        return a0;
    }
#endif
