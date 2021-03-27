module Opcode exposing (Opcode(..), OpcodeData)


type alias OpcodeData =
    { bgColor : String
    , pneumonic : String
    , size : Int
    , timeTaken : String
    , flags : String
    }


type Opcode
    = Set OpcodeData
    | Unset
