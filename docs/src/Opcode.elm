module Opcode exposing (Opcode(..), OpcodeData)


type alias OpcodeData =
    { pneumonic : String
    , color : String
    , size : Int
    , timeTaken : String
    , flags : String
    }


type Opcode
    = Set OpcodeData
    | Unset
