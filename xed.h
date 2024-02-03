
// xed-static.c depends on having the types from stdint.h available but does
// not include it explicitly. This works fine on linux where XED includes
// stdint but not on windows where it does not.
#include <stdint.h>

#include "xed/xed-interface.h"
#include "xed/xed-isa-set.h"

#ifdef XED_SYS_ENC2_M32_A32
#   include "xed/xed-enc2-m32-a32.h"
#   ifdef XED_SYS_ENC2_CHK
#       include "xed/xed-chk-enc2-m32-a32.h"
#   endif
#endif

#ifdef XED_SYS_ENC2_M64_A64
#   include "xed/xed-enc2-m64-a64.h"
#   ifdef XED_SYS_ENC2_CHK
#       include "xed/xed-chk-enc2-m64-a64.h"
#   endif
#endif
