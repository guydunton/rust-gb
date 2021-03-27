#!/bin/bash -eux

# This script just builds docs/src/SupportedCodes.elm.
# Prerequisites:
#   Run from the docs directory

# Pull out all the opcodes from the file
SUPPORTED_CODES=$(cat ../../src/gameboy/opcodes/opcodes.rs \
    | grep -E '0x[0-9A-F][0-9A-F]' \
    | sed 's/[[:space:]]*(\(0x[0-9A-F][0-9A-F]\).*/"\1",/g')

# Remove the last ,
SUPPORTED_CODES="${SUPPORTED_CODES%?}"

# Put into a single line
SUPPORTED_CODES=$(echo $SUPPORTED_CODES)

# Repeat for the CB opcodes
CB_CODES=$(cat ../../src/gameboy/opcodes/cb_opcodes.rs \
    | grep -E '0x[0-9A-F][0-9A-F]' \
    | sed 's/[[:space:]]*(\(0x[0-9A-F][0-9A-F]\).*/"\1",/g')
CB_CODES="${CB_CODES%?}"
CB_CODES=$(echo $CB_CODES)

# Replace both parts in the template file and 
# run through the elm formatter before saving into
# the correct directory
sed "s/{supportedCodes}/${SUPPORTED_CODES}/g" src/SupportedCodes.elm.template \
| sed "s/{supportedCBCodes}/${CB_CODES}/g" \
| elm-format --stdin \
> src/SupportedCodes.elm
