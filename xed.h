
// xed-static.c depends on having the types from stdint.h available but does
// not include it explicitly. This works fine on linux where XED includes
// stdint but not on windows where it does not.
#include <stdint.h>

#include "xed/xed-interface.h"
#include "xed/xed-isa-set.h"

#ifdef XED_ENC2_ENCODER
#include "xed/xed-enc2-m32-a32.h"
#include "xed/xed-chk-enc2-m32-a32.h"
#include "xed/xed-enc2-m64-a64.h"
#include "xed/xed-chk-enc2-m64-a64.h"
#endif
