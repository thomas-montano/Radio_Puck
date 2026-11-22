/* STM32F407VGT6 */
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 1024K
  /* SRAM1 + SRAM2. The extra 64K of CCM RAM at 0x10000000 is left unused
     (it can't be reached by DMA). */
  RAM   : ORIGIN = 0x20000000, LENGTH = 128K
}
