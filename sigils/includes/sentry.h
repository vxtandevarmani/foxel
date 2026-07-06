#pragma once

#include "platform.h"

struct packet {
    uintptr_t base;
    uintptr_t length;
};


struct param {
    char* name;
    uint32_t length;
};


namespace sigil {
    class instance {
        struct {
            uintptr_t address;
            uintptr_t length;
        } base = {};

        functions api;

        public:
            explicit instance(void*);
            void main(void*);
            int facto(const char*);
    };
}
