#include "../includes/sentry.h"

using namespace sigil;


int instance::facto(const char* buffer) {
    int cnter = 0;
    for (const char* p = buffer; *p; ++p) {
        ++cnter;
    }
    return cnter+1;
}
