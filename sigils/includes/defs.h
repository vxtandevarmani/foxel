#pragma once

#include <stdint.h>
#include <stddef.h>

typedef uint8_t     BYTE;
typedef uint16_t    WORD;
typedef uint32_t    DWORD;
typedef uint64_t    QWORD;
typedef void*       PVOID;
typedef void*       HANDLE;
typedef void*       HMODULE;
typedef void*       HWND;
typedef BYTE*       PBYTE;
typedef WORD*       PWORD;
typedef DWORD*      PDWORD;
typedef const char* LPCSTR;
typedef int         BOOL;
typedef uint32_t    UINT;
typedef PVOID       FARPROC;


typedef struct _UNICODE_STRING {
    WORD    Length;
    WORD    MaximumLength;
    char16_t* Buffer;
} UNICODE_STRING, *PUNICODE_STRING;

typedef struct _LIST_ENTRY {
    struct _LIST_ENTRY* Flink;
    struct _LIST_ENTRY* Blink;
} LIST_ENTRY, *PLIST_ENTRY;

typedef struct _LDR_DATA_TABLE_ENTRY {
    LIST_ENTRY  InLoadOrderLinks;
    LIST_ENTRY  InMemoryOrderLinks;
    LIST_ENTRY  InInitializationOrderLinks;
    PVOID       DllBase;
    PVOID       EntryPoint;
    DWORD       SizeOfImage;
    UNICODE_STRING FullDllName;
    UNICODE_STRING BaseDllName;
} LDR_DATA_TABLE_ENTRY, *PLDR_DATA_TABLE_ENTRY;

typedef struct _PEB_LDR_DATA {
    DWORD       Length;
    BOOL        Initialized;
    PVOID       SsHandle;
    LIST_ENTRY  InLoadOrderModuleList;
    LIST_ENTRY  InMemoryOrderModuleList;
    LIST_ENTRY  InInitializationOrderModuleList;
} PEB_LDR_DATA, *PPEB_LDR_DATA;

typedef struct _PEB {
    BYTE            Reserved1[2];
    BYTE            BeingDebugged;
    BYTE            Reserved2[1];
    PVOID           Reserved3[2];
    PPEB_LDR_DATA   Ldr;
} PEB, *PPEB;

typedef struct _IMAGE_DOS_HEADER {
    WORD e_magic;
    WORD e_cblp;
    WORD e_cp;
    WORD e_crlc;
    WORD e_cparhdr;
    WORD e_minalloc;
    WORD e_maxalloc;
    WORD e_ss;
    WORD e_sp;
    WORD e_csum;
    WORD e_ip;
    WORD e_cs;
    WORD e_lfarlc;
    WORD e_ovno;
    WORD e_res[4];
    WORD e_oemid;
    WORD e_oeminfo;
    WORD e_res2[10];
    DWORD e_lfanew;
} IMAGE_DOS_HEADER, *PIMAGE_DOS_HEADER;

typedef struct _IMAGE_FILE_HEADER {
    WORD  Machine;
    WORD  NumberOfSections;
    DWORD TimeDateStamp;
    DWORD PointerToSymbolTable;
    DWORD NumberOfSymbols;
    WORD  SizeOfOptionalHeader;
    WORD  Characteristics;
} IMAGE_FILE_HEADER, *PIMAGE_FILE_HEADER;

typedef struct _IMAGE_DATA_DIRECTORY {
    DWORD VirtualAddress;
    DWORD Size;
} IMAGE_DATA_DIRECTORY, *PIMAGE_DATA_DIRECTORY;

#define IMAGE_NUMBEROF_DIRECTORY_ENTRIES 16
#define IMAGE_DIRECTORY_ENTRY_EXPORT      0

typedef struct _IMAGE_OPTIONAL_HEADER {
    WORD    Magic;
    BYTE    MajorLinkerVersion;
    BYTE    MinorLinkerVersion;
    DWORD   SizeOfCode;
    DWORD   SizeOfInitializedData;
    DWORD   SizeOfUninitializedData;
    DWORD   AddressOfEntryPoint;
    DWORD   BaseOfCode;
    QWORD   ImageBase;
    DWORD   SectionAlignment;
    DWORD   FileAlignment;
    WORD    MajorOperatingSystemVersion;
    WORD    MinorOperatingSystemVersion;
    WORD    MajorImageVersion;
    WORD    MinorImageVersion;
    WORD    MajorSubsystemVersion;
    WORD    MinorSubsystemVersion;
    DWORD   Win32VersionValue;
    DWORD   SizeOfImage;
    DWORD   SizeOfHeaders;
    DWORD   CheckSum;
    WORD    Subsystem;
    WORD    DllCharacteristics;
    QWORD   SizeOfStackReserve;
    QWORD   SizeOfStackCommit;
    QWORD   SizeOfHeapReserve;
    QWORD   SizeOfHeapCommit;
    DWORD   LoaderFlags;
    DWORD   NumberOfRvaAndSizes;
    IMAGE_DATA_DIRECTORY DataDirectory[IMAGE_NUMBEROF_DIRECTORY_ENTRIES];
} IMAGE_OPTIONAL_HEADER, *PIMAGE_OPTIONAL_HEADER;

typedef struct _IMAGE_NT_HEADERS {
    DWORD               Signature;
    IMAGE_FILE_HEADER   FileHeader;
    IMAGE_OPTIONAL_HEADER OptionalHeader;
} IMAGE_NT_HEADERS, *PIMAGE_NT_HEADERS;

typedef struct _IMAGE_EXPORT_DIRECTORY {
    DWORD Characteristics;
    DWORD TimeDateStamp;
    WORD  MajorVersion;
    WORD  MinorVersion;
    DWORD Name;
    DWORD Base;
    DWORD NumberOfFunctions;
    DWORD NumberOfNames;
    DWORD AddressOfFunctions;
    DWORD AddressOfNames;
    DWORD AddressOfNameOrdinals;
} IMAGE_EXPORT_DIRECTORY, *PIMAGE_EXPORT_DIRECTORY;

FARPROC GetProcAddress(
  [in] HMODULE hModule,
  [in] LPCSTR  lpProcName
);

HMODULE LoadLibraryA(
  [in] LPCSTR lpLibFileName
);

int MessageBoxA(
  [in, optional] HWND    hWnd,
  [in, optional] LPCSTR lpText,
  [in, optional] LPCSTR lpCaption,
  [in]           UINT    uType
);

