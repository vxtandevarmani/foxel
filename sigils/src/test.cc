#include "../includes/sentry.h"

using namespace sigil;

extern "C" void _start(void* reloc, void* args) {
    sigil::instance().main(reloc, args);
    return;
}

#define RESOLVE_FN(api, mod, fn)                    \
    api.mod.fn = reinterpret_cast<decltype(fn)*>(   \
        utils::resolve::grab_fn_ptr(                \
            api.mod.handle,                         \
            utils::expr::hash_string(#fn)           \
        )                                           \
    )

/*
Note to self i need to build a build system
that changes the linker script 2nd section between
.rdata (win) and .rodata (lin)
*/

instance::instance(void) {

    api.kernel32.handle = utils::resolve::grab_dll_ptr(
        utils::expr::hash_string<char16_t>(u"kernel32.dll")
    );

    RESOLVE_IMPORT(api.kernel32);

}  // this can be used for resolving any of the external libraries?? or calls or whatever the fuck you wanna do

void instance::main(void* reloc, void* args) {
    struct packet* pckt = (packet*)(reloc);
    base.address = pckt->base;
    base.length = pckt->length;

    struct param* prms = (param*)(args);


    const char* sgst = "I <3 Breaking ..";
    int sze = facto(sgst);
    if(sze != prms->length){
        prms->name = "Hello Worl";
    }

    utils::memory::cpy((void*)prms->name, (void*)sgst, sze);

    //api.user32.handle = (uintptr_t)api.kernel32.LoadLibraryA("user32.dll");
    
    uint64_t arg[13];
    utils::memory::zro(arg, sizeof(arg));
    arg[0] = (uint64_t)("user32.dll");

    api.user32.handle = host_call((uintptr_t)api.kernel32.LoadLibraryA, arg);

    //RESOLVE_IMPORT(api.user32);


    arg[0] = NULL;
    arg[1] = (uint64_t)("Hello WORLD");
    arg[2] = (uint64_t)("TEST");
    arg[3] = 0;

    host_call((uintptr_t)api.user32.MessageBoxA, arg);

    //api.user32.MessageBoxA = utils::resolve::grab_fn_ptr(
    //                        api.user32.handle,
    //                        utils::expr::hash_string("MessageBoxA")
    //                    );


    //api.user32.MessageBoxA(NULL, "Hello, World!", "Test", 0);


    return;
}