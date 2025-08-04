################################################################################
# Automatically-generated file. Do not edit!
# Toolchain: GNU Tools for STM32 (13.3.rel1)
################################################################################

# Add inputs and outputs from these tool invocations to the build variables 
C_SRCS += \
../Core/Src/background.c \
../Core/Src/display.c \
../Core/Src/font.c \
../Core/Src/i3g4250d.c \
../Core/Src/ili9341.c \
../Core/Src/main.c \
../Core/Src/obstacle.c \
../Core/Src/player.c \
../Core/Src/stm32f4xx_hal_msp.c \
../Core/Src/stm32f4xx_it.c \
../Core/Src/syscalls.c \
../Core/Src/sysmem.c \
../Core/Src/system_stm32f4xx.c \
../Core/Src/text_msg.c 

OBJS += \
./Core/Src/background.o \
./Core/Src/display.o \
./Core/Src/font.o \
./Core/Src/i3g4250d.o \
./Core/Src/ili9341.o \
./Core/Src/main.o \
./Core/Src/obstacle.o \
./Core/Src/player.o \
./Core/Src/stm32f4xx_hal_msp.o \
./Core/Src/stm32f4xx_it.o \
./Core/Src/syscalls.o \
./Core/Src/sysmem.o \
./Core/Src/system_stm32f4xx.o \
./Core/Src/text_msg.o 

C_DEPS += \
./Core/Src/background.d \
./Core/Src/display.d \
./Core/Src/font.d \
./Core/Src/i3g4250d.d \
./Core/Src/ili9341.d \
./Core/Src/main.d \
./Core/Src/obstacle.d \
./Core/Src/player.d \
./Core/Src/stm32f4xx_hal_msp.d \
./Core/Src/stm32f4xx_it.d \
./Core/Src/syscalls.d \
./Core/Src/sysmem.d \
./Core/Src/system_stm32f4xx.d \
./Core/Src/text_msg.d 


# Each subdirectory must supply rules for building sources it contributes
Core/Src/%.o Core/Src/%.su Core/Src/%.cyclo: ../Core/Src/%.c Core/Src/subdir.mk
	arm-none-eabi-gcc "$<" -mcpu=cortex-m4 -std=gnu11 -g3 -DDEBUG -DUSE_HAL_DRIVER -DSTM32F429xx -c -I../Core/Inc -I../Drivers/STM32F4xx_HAL_Driver/Inc -I../Drivers/STM32F4xx_HAL_Driver/Inc/Legacy -I../Drivers/CMSIS/Device/ST/STM32F4xx/Include -I../Drivers/CMSIS/Include -O0 -ffunction-sections -fdata-sections -Wall -fstack-usage -fcyclomatic-complexity -MMD -MP -MF"$(@:%.o=%.d)" -MT"$@" --specs=nano.specs -mfpu=fpv4-sp-d16 -mfloat-abi=hard -mthumb -o "$@"

clean: clean-Core-2f-Src

clean-Core-2f-Src:
	-$(RM) ./Core/Src/background.cyclo ./Core/Src/background.d ./Core/Src/background.o ./Core/Src/background.su ./Core/Src/display.cyclo ./Core/Src/display.d ./Core/Src/display.o ./Core/Src/display.su ./Core/Src/font.cyclo ./Core/Src/font.d ./Core/Src/font.o ./Core/Src/font.su ./Core/Src/i3g4250d.cyclo ./Core/Src/i3g4250d.d ./Core/Src/i3g4250d.o ./Core/Src/i3g4250d.su ./Core/Src/ili9341.cyclo ./Core/Src/ili9341.d ./Core/Src/ili9341.o ./Core/Src/ili9341.su ./Core/Src/main.cyclo ./Core/Src/main.d ./Core/Src/main.o ./Core/Src/main.su ./Core/Src/obstacle.cyclo ./Core/Src/obstacle.d ./Core/Src/obstacle.o ./Core/Src/obstacle.su ./Core/Src/player.cyclo ./Core/Src/player.d ./Core/Src/player.o ./Core/Src/player.su ./Core/Src/stm32f4xx_hal_msp.cyclo ./Core/Src/stm32f4xx_hal_msp.d ./Core/Src/stm32f4xx_hal_msp.o ./Core/Src/stm32f4xx_hal_msp.su ./Core/Src/stm32f4xx_it.cyclo ./Core/Src/stm32f4xx_it.d ./Core/Src/stm32f4xx_it.o ./Core/Src/stm32f4xx_it.su ./Core/Src/syscalls.cyclo ./Core/Src/syscalls.d ./Core/Src/syscalls.o ./Core/Src/syscalls.su ./Core/Src/sysmem.cyclo ./Core/Src/sysmem.d ./Core/Src/sysmem.o ./Core/Src/sysmem.su ./Core/Src/system_stm32f4xx.cyclo ./Core/Src/system_stm32f4xx.d ./Core/Src/system_stm32f4xx.o ./Core/Src/system_stm32f4xx.su ./Core/Src/text_msg.cyclo ./Core/Src/text_msg.d ./Core/Src/text_msg.o ./Core/Src/text_msg.su

.PHONY: clean-Core-2f-Src

