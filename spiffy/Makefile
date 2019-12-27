
APP=spiffy
CC=arm-none-eabi-gcc
CFLAGS = -mthumb -mcpu=cortex-m4 -Wall -Wextra
CFLAGS += -O0 -g3
CFLAGS += -nostdlib
LDFLAGS = -Tstm32_flash.ld

all: spiffy.bin spiffy.hex

spiffy.elf: boot.s main.c Makefile stm32_flash.ld
	${CC} ${CFLAGS} ${LDFLAGS} boot.s main.c -o ${APP}.elf
	arm-none-eabi-objdump -d ${APP}.elf

${APP}.bin: spiffy.elf Makefile
	arm-none-eabi-objcopy -O binary ${APP}.elf ${APP}.bin
	hexdump -C ${APP}.bin

${APP}.hex: ${APP}.elf Makefile
	arm-none-eabi-objcopy -O ihex ${APP}.elf ${APP}.hex

flash: ${APP}.bin
	st-flash write ${APP}.bin 0x8000000

debug: ${APP}.elf
	echo "Make sure you run st-util in a seperate console!!!"
	arm-none-eabi-gdb ${APP}.elf
