add_executable(mtp-hotplug mtp-hotplug.c)
target_link_libraries(mtp-hotplug mtp)
target_include_directories(mtp-hotplug PRIVATE "../include")
