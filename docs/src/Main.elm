module Main exposing (main)

import Html exposing (Html, p, table, td, text, tr)
import Opcode exposing (Opcode(..))
import OpcodesDict exposing (opcodes_dict)


split : Int -> List a -> List (List a)
split i list =
    case List.take i list of
        [] ->
            []

        listHead ->
            listHead :: split i (List.drop i list)


printOpcode : Opcode -> Html ()
printOpcode opcode =
    case opcode of
        Set data ->
            td [] [ text data.pneumonic ]

        Unset ->
            td [] [ text "unset" ]


convertRow : List (Html ()) -> Html ()
convertRow row =
    tr [] row


convertToTable : List (Html ()) -> Html ()
convertToTable rows =
    table [] rows


main =
    -- Create all table coords
    -- Loop through table coords
    opcodes_dict
        |> List.map printOpcode
        |> split 16
        |> List.map convertRow
        |> convertToTable
