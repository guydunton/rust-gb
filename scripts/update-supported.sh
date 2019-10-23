#!/bin/bash -eux

SUPPORTED_CODES=$(cat src/gameboy/opcodes/opcodes.rs \
    | grep -E '0x[0-9A-F][0-9A-F] =>' \
    | sed 's/[[:space:]]*\(0x[0-9A-F][0-9A-F]\).*/"\1",/g')

# Remove the last ,
SUPPORTED_CODES="${SUPPORTED_CODES%?}"

# Put into a single line
SUPPORTED_CODES=$(echo $SUPPORTED_CODES)

CB_CODES=$(cat src/gameboy/opcodes/cb_opcodes.rs \
    | grep -E '0x[0-9A-F][0-9A-F] =>' \
    | sed 's/[[:space:]]*\(0x[0-9A-F][0-9A-F]\).*/"\1",/g')
CB_CODES="${CB_CODES%?}"
CB_CODES=$(echo $CB_CODES)

sed "s/{supportedCodes}/${SUPPORTED_CODES}/g" docs/src/SupportedCodes.elm.template \
| sed "s/{supportedCBCodes}/${CB_CODES}/g" \
| elm-format --stdin \
> docs/src/SupportedCodes.elm

# Update git
git update-index -q --refresh
if ! git diff-index --quiet HEAD --; then
    # There are changes
    echo "There are changes. Deploying new version"
    git add docs/src/SupportedCodes.elm
    git commit -m "Automated commit of supported codes"
    git remote rm origin
    git remote add origin https://guydunton:$GITHUB_TOKEN@github.com/guydunton/rust-gb.git
    git push
else
    echo "Nothing has changed. Not committing"
fi
