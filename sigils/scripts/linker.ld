
ENTRY(_start)
SECTIONS
{
    .text :
    {
        *(.text*)
        __my_boundry = .;
        *(.rodata*)
        LONG(__my_boundry);
    }
}
