#include "../includes/sentry.h"

using namespace sigil;

extern "C" void _start(void* reloc, void* args) {
    sigil::instance(reloc).main(args);
    return;
}


// here to make the linker happy
extern "C" void * memset (void *dest, int val, size_t len) {
    unsigned char *ptr = (unsigned char *)dest;
    while (len-- > 0)
      *ptr++ = val;
    return dest;
}

// here to make the linker happy
extern "C" void * memcpy (void *dest, const void *src, size_t len) {
    char *d = (char*) dest;
    const char *s = (const char*)src;
    while (len--)
      *d++ = *s++;
    return dest;
}

/*
  Note to self i need to build a build system
  that changes the linker script 2nd section between
  .rdata (win) and .rodata (lin)
*/

instance::instance(void* relocs) {

    struct packet* pckt = (packet*)(relocs);
    base.address = pckt->base;
    base.length = pckt->length;

    api.kernel32.handle = utils::resolve::grab_dll_ptr(
        utils::expr::hash_string<char16_t>(u"kernel32.dll")
    );

    api.kernel32.LoadLibraryA = reinterpret_cast<decltype(LoadLibraryA)*>(
    utils::resolve::grab_fn_ptr(
        utils::expr::hash_string<char>("LoadLibraryA"),
        api.kernel32.handle
    ));

}  // this can be used for resolving any of the external libraries?? or calls or whatever the heck you wanna do

void instance::main(void* args) {
    
    
    struct param* prms = (param*)(args);
    const char* sgst = "I <3 Breaking ..";
    int size = facto(sgst);
    if(size != prms->length) {
        prms->name = "Hello Worl";
    }
    utils::memory::cpy((void*)prms->name, (void*)sgst, size);


    uint64_t arg[13] = {0};
    //utils::memory::zro(arg, sizeof(arg));
    arg[0] = (uint64_t)("user32.dll");

    api.user32.handle = host_call((uintptr_t)api.kernel32.LoadLibraryA, arg);

    api.user32.MessageBoxA = reinterpret_cast<decltype(MessageBoxA)*>(
    
    utils::resolve::grab_fn_ptr(
        utils::expr::hash_string<char>("MessageBoxA"),
        api.user32.handle
    ));
    
    arg[0] = NULL;
    arg[1] = (uint64_t)("I AM A LONELEYY VIRTUAL MACHINEEE");
    arg[2] = (uint64_t)("TEST");
    arg[3] = 0x00000030L | 0x00000006L | 0x00000100L;

    host_call((uintptr_t)api.user32.MessageBoxA, arg);
    
    return;

}