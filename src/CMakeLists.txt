find_package(PkgConfig)

pkg_check_modules(LIBUSB REQUIRED libusb-1.0)

find_package(GCrypt)

if(NOT ${GCRYPT_FOUND})
    message(FATAL_ERROR "Cannot find GCrypt")
endif(NOT ${GCRYPT_FOUND})

set(MTP_SOURCE_FILES
    "libmtp.c"
    "libusb1-glue.c"
    "mtpz.c"
    "playlist-spl.c"
    "ptp-pack.c"
    "ptp.c"
    "unicode.c"
    "util.c")

add_library(mtp STATIC ${MTP_SOURCE_FILES})
target_link_libraries(mtp ${LIBUSB_LIBRARIES} ${GCRYPT_LIBRARIES})
target_include_directories(mtp PUBLIC ${LIBUSB_INCLUDE_DIRS} ${GCRYPT_INCLUDE_DIRS} "../include")
