#ifndef __EMULATOR_H__
#define __EMULATOR_H__

#define CLOCK_180_MHZ 0
#define STAY_IN_BOOTLOADER_FLAG 0x0FC35A96
#define mini_snprintf snprintf

#undef FIRMWARE_START

#include <stdint.h>
#include <stdio.h>

extern uint8_t *FIRMWARE_START;
extern uint32_t stay_in_bootloader_flag;

void emulator_poll_events(void);
void set_core_clock(int);
void mpu_config_bootloader(void);
void mpu_config_off(void);
void display_set_little_endian(void);
void jump_to(void *addr);
void ensure_compatible_settings(void);

#endif
