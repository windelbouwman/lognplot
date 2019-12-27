
ENTRY(Reset_Handler)

_Stack_Size = 0x1000;

MEMORY
{
    FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 1024K
    RAM (xrw)  : ORIGIN = 0x20000000, LENGTH = 192K
}

SECTIONS
{
    .isr_vector : 
    {
        . = ALIGN(4);
        KEEP(*(.isr_vector))
        . = ALIGN(4);
    } >FLASH

    .text : 
    {
        . = ALIGN(4);
        *(.text)
        . = ALIGN(4);
    } >FLASH

    .data : {
        . = ALIGN(4);
        *(.data)
        . = ALIGN(4);
    } >RAM

    .bss : {
        . = ALIGN(4);
        *(.bss)
        . = ALIGN(4);
    } >RAM

    .stack : {
        . = ALIGN(4);
        . = . + _Stack_Size;
        PROVIDE(_estack = .);
    } >RAM
}
