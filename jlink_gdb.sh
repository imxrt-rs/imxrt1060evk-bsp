#!/bin/sh

# Start a jlink gdb server with the correct flags for imxrt1062evk

JLinkGDBServer -device MIMXRT1062xxx6A -if swd
