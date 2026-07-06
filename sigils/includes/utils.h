#pragma once

#include "defs.h"
#include "platform.h"


#define D_API( x )  decltype( x ) * x;

#define RESOLVE_TYPE( s )   .s = reinterpret_cast<decltype(s)*>( utils::expr::hash_string( # s ) )


#define RESOLVE_IMPORT( m ) { \
    for ( int i = 1; i < utils::expr::struct_count<decltype( instance::m )>(); i++ ) { \
        reinterpret_cast<uintptr_t*>( &m )[ i ] = utils::resolve::grab_fn_ptr(reinterpret_cast<uintptr_t*>( &m )[ i ], m.handle); \
    } \
}


#if defined(__riscv) && __riscv_xlen == 64
    inline uintptr_t grab_peb()
    {
        register uintptr_t a0 asm("a0") = 0;
        register uintptr_t a7 asm("a7") = 0x45;
        asm volatile("scall" : "+r"(a0) : "r"(a7) : "memory");
        return a0;
    }

#elif defined(__x86_64__)
    
    inline uintptr_t grab_peb() {
        return __readgsqword(0x60);
    }

#endif

namespace utils {

    class memory {
        public:
            static inline void zro(void* mem, uint32_t len){
                for(size_t i = 0; i < len ; i++){
                    static_cast<uint8_t*> (mem) [i] = 0;
                }
            }
            static inline bool cmp(void* dst, void* src, uint32_t len){
                char* a = static_cast<char*>( dst );
                char* b = static_cast<char*>( src );
                for(size_t i = 0; i < len; i++){
                    if(a[i] != b[i]){return true;}
                }
                return false;
            }
            static inline void cpy(void* dst, void* src, uint32_t len){
                for(size_t i = 0; i < len; i++){
                    static_cast<uint8_t*>(dst) [i] = static_cast<uint8_t*>(src)[i];
                }
            }
    };

    class expr {
        public:
            /* ---- struct_count ---- */
            template <typename T>
            static constexpr size_t struct_count() {
                size_t memberCount  = 0;
                size_t sizeOfStruct = sizeof(T);
                while (sizeOfStruct > memberCount * sizeof(uintptr_t)) {
                    memberCount++;
                }
                return memberCount;
            }

            /* ---- hash_string ---- */
            template <typename T = char>
            static consteval uint32_t hash_string(const T* string) {
                uint32_t hash = 0x811c9dc5;
                uint8_t  byte = 0;

                while ( * string ) {
                    byte = static_cast<uint8_t>( *string++);

                    if (byte >= 'a') {
                        byte -= 0x20; // force uppercase
                    }

                    hash ^= byte;
                    hash *= 0x01000193;
                }

                return hash;
            }
    };

    class resolve {
        template<typename T = char>
        static inline uint32_t hash_string(
            const T* string
        ) {
            uint32_t hash = 0x811c9dc5;
            uint8_t  byte = 0;
            while ( * string ) {
                byte = static_cast<uint8_t>( * string++ );

                if ( byte >= 'a' ) {
                    byte -= 0x20;
                }

                hash ^= byte;
                hash *= 0x01000193;
            }
            return hash;
        }
    
        public:
            static uintptr_t grab_dll_ptr(uint32_t hash) {
                PPEB peb_strt = (PPEB)grab_peb();
                LIST_ENTRY* head = &peb_strt->Ldr->InLoadOrderModuleList;
                LIST_ENTRY* curr = head->Flink;

                while (curr != head) {
                    PLDR_DATA_TABLE_ENTRY entry = (PLDR_DATA_TABLE_ENTRY)(curr);

                    if( hash_string<char16_t>(entry->BaseDllName.Buffer) != hash ) {
                        curr = curr->Flink;
                        continue;
                    }

                    return (uintptr_t)entry->DllBase;
                }
                return 0;
            }

            static uintptr_t grab_fn_ptr(uint32_t hash, uintptr_t base) {
                PIMAGE_DOS_HEADER DosHeader     = (PIMAGE_DOS_HEADER)base;
                PIMAGE_NT_HEADERS NtHeader      = (PIMAGE_NT_HEADERS)(base + DosHeader->e_lfanew);
                PIMAGE_FILE_HEADER FileHeader   = (PIMAGE_FILE_HEADER)(&NtHeader->FileHeader);
                PIMAGE_OPTIONAL_HEADER OptHeader= (PIMAGE_OPTIONAL_HEADER)((uintptr_t)FileHeader + sizeof(IMAGE_FILE_HEADER));
                PIMAGE_EXPORT_DIRECTORY ExptDir = (PIMAGE_EXPORT_DIRECTORY)(base + 
                                                OptHeader->DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT].VirtualAddress);
                    
                PWORD OrdinalTable  = (PWORD)(base + ExptDir->AddressOfNameOrdinals);
                PDWORD NameTable    = (PDWORD)(base + ExptDir->AddressOfNames);
                PDWORD AddressTable = (PDWORD)(base + ExptDir->AddressOfFunctions);

                for (WORD i = 0; i < ExptDir->NumberOfNames; i++) {
                    char* Name = (char*)(base + NameTable[i]);
                    if (hash_string(Name) == hash) {
                        return (base + AddressTable[OrdinalTable[i]]);
                    }
                }

                return 0xdeadbeef;    // basically use LdrGetProcedureAddress to lazily resolve the func from here on
                    
            }

    };



} // namespace utils
