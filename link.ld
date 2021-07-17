ENTRY(_start)

ROUTER_STACK_SIZE = 4096;
SHELL_STACK_SIZE  = 4096;
SVC_STACK_SIZE    = 1024;
SYS_STACK_SIZE    = 1024;

SECTIONS{
    . = 0x8000;
    
    .text :
    {
        KEEP(*(.text.boot))
        *(.text .text.*)
    }
    . = ALIGN(4096);
    
    .rodata :
    {
        *(.rodata)
    }
    . = ALIGN(4096);
    
    .data :
    {
        *(.data)
    }
    . = ALIGN(4096); 
     
    __bss_start = .;
    .bss :
    {
        bss = .;
        *(.bss)
    }
    . = ALIGN(4096);
    __bss_end = .;

    .kernelheap : {
        __kernel_heap_start__ = .;
        . += 0x100000;
    }

    . = ALIGN(4096);
    __kernel_heap_end__ = .;

    .stack : {

        . += ROUTER_STACK_SIZE;
        . = ALIGN(4);
        __router_stack_end = .;

        . += SHELL_STACK_SIZE;
        . = ALIGN(4);
        __shell_stack_end = .;

        . += SVC_STACK_SIZE;
        . = ALIGN(4);
        __svc_stack_end = .;

        . += SYS_STACK_SIZE;
        . = ALIGN(4);
        __sys_stack_end = .;
    }
    __end = .;

}