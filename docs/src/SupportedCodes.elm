module SupportedCodes exposing (isCoordSupported, supportedCodes)

import Hex exposing (toHex)


supportedCodes : List String
supportedCodes =
    [ "0x00"
    , "0x05"
    , "0x06"
    , "0x0C"
    , "0x0E"
    , "0x11"
    , "0x17"
    , "0x1A"
    , "0x20"
    , "0x21"
    , "0x22"
    , "0x23"
    , "0x31"
    , "0x32"
    , "0x3E"
    , "0x4F"
    , "0x77"
    , "0xAF"
    , "0xC1"
    , "0xC5"
    , "0xCB"
    , "0xCD"
    , "0xE0"
    , "0xE2"
    ]


isCoordSupported : ( Int, Int ) -> Bool
isCoordSupported ( x, y ) =
    let
        xChar =
            toHex x

        yChar =
            toHex y

        hexCoord =
            "0x" ++ yChar ++ xChar
    in
    List.member hexCoord supportedCodes
