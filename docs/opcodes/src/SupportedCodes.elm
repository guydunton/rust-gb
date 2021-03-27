module SupportedCodes exposing (isCoordSupported, supportedCBCodes, supportedCodes)

import Hex exposing (toHex)


supportedCodes : List String
supportedCodes =
    [ "0x00"
    , "0x04"
    , "0x05"
    , "0x06"
    , "0x0C"
    , "0x0D"
    , "0x0E"
    , "0x11"
    , "0x13"
    , "0x15"
    , "0x17"
    , "0x18"
    , "0x1A"
    , "0x1D"
    , "0x1E"
    , "0x20"
    , "0x21"
    , "0x22"
    , "0x23"
    , "0x24"
    , "0x28"
    , "0x2E"
    , "0x31"
    , "0x32"
    , "0x3D"
    , "0x3E"
    , "0x4F"
    , "0x57"
    , "0x67"
    , "0x77"
    , "0x7B"
    , "0x7C"
    , "0x90"
    , "0xAF"
    , "0xC1"
    , "0xC5"
    , "0xC9"
    , "0xCD"
    , "0xE0"
    , "0xE2"
    , "0xEA"
    , "0xF0"
    , "0xFE"
    ]


supportedCBCodes : List String
supportedCBCodes =
    [ "0x11"
    , "0x7C"
    ]


isCoordSupported : List String -> ( Int, Int ) -> Bool
isCoordSupported supported ( x, y ) =
    let
        xChar =
            toHex x

        yChar =
            toHex y

        hexCoord =
            "0x" ++ yChar ++ xChar
    in
    List.member hexCoord supported
