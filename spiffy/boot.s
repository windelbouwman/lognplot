.cpu cortex-m4
.thumb

 .section .text
 .global Reset_Handler
 .type  Reset_Handler, %function

Reset_Handler:
  bl  main

 .section  .isr_vector,"a",%progbits

isr_vectors:
  .word _estack
  .word Reset_Handler

