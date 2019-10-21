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


convertRow : Int -> List (Html ()) -> Html ()
convertRow index row =
    tr [] ([ td [] [ text (String.fromInt index) ] ] ++ row)


convertToTable : List (Html ()) -> Html ()
convertToTable rows =
    let
        headerRow =
            List.range 0 15
                |> List.map (\i -> td [] [ text (String.fromInt i) ])
    in
    table [] ((td [] [] :: headerRow) ++ rows)


main =
    -- Create all table coords
    -- Loop through table coords
    opcodes_dict
        |> List.map printOpcode
        |> split 16
        |> List.indexedMap convertRow
        |> convertToTable
