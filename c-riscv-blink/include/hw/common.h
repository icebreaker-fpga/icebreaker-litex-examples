#ifndef __HW_COMMON_H
#define __HW_COMMON_H

#include <stdint.h>

/* To overwrite CSR accessors, define extern, non-inlined versions
 * of csr_read[bwl]() and csr_write[bwl](), and define
 * CSR_ACCESSORS_DEFINED.
 */

#ifndef CSR_ACCESSORS_DEFINED
#define CSR_ACCESSORS_DEFINED

#ifdef __ASSEMBLER__
#define MMPTR(x) x
#else /* ! __ASSEMBLER__ */
#define MMPTR(x) (*((volatile unsigned int *)(x)))

static inline void csr_write_simple(uint32_t value, uint32_t addr)
{
	*((volatile uint32_t *)addr) = value;
}

static inline uint32_t csr_read_simple(uint32_t addr)
{
	return *(volatile uint32_t *)addr;
}

#endif /* ! __ASSEMBLER__ */

#endif /* ! CSR_ACCESSORS_DEFINED */

#endif /* __HW_COMMON_H */
