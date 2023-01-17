// #include <bzlib.h>
// #define UINT16_MAX 0xffff       /* 65535U */
#include <stdint.h>

// #include "bacnet-stack/src/bacnet/bacdef.h"
// #include "bacnet-stack/src/bacnet/datalink/datalink.h"
// #include "bacnet-stack/src/bacnet/apdu.h"
// #include "bacnet-stack/src/bacnet/bacenum.h"
#include "bacnet/bacdef.h"
#include "bacnet/config.h"
#include "bacnet/bactext.h"
#include "bacnet/bacerror.h"
#include "bacnet/iam.h"
#include "bacnet/arf.h"
#include "bacnet/basic/tsm/tsm.h"
#include "bacnet/basic/binding/address.h"
#include "bacnet/npdu.h"
#include "bacnet/apdu.h"
#include "bacnet/basic/object/device.h"
#include "bacnet/datalink/datalink.h"
#include "bacnet/whois.h"
#include "bacnet/version.h"
#include "bacnet/basic/sys/filename.h"
#include "bacnet/basic/services.h"
#include "bacnet/basic/tsm/tsm.h"
#include "bacnet/datalink/dlenv.h"
// #include "bacport.h"