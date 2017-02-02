add_executable(mtp-hotplug mtp-hotplug.c)
target_link_libraries(mtp-hotplug mtp)
target_include_directories(mtp-hotplug PRIVATE "../include")

add_executable(mtp-probe mtp-probe.c)
target_link_libraries(mtp-probe mtp)
target_include_directories(mtp-probe PRIVATE "../include")
