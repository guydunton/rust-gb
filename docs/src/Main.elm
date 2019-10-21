module Main exposing (main)

import Html exposing (Html, br, p, span, table, td, text, tr)
import Html.Attributes exposing (style)
import Opcode exposing (Opcode(..), OpcodeData)
import OpcodesDict exposing (opcodes_dict)


split : Int -> List a -> List (List a)
split i list =
    case List.take i list of
        [] ->
            []

        listHead ->
            listHead :: split i (List.drop i list)


cellStyles : String -> List (Html.Attribute msg)
cellStyles color =
    [ style "background-color" color
    , style "border-width" "1px"
    , style "border-style" "solid"
    , style "border-color" "black"
    , style "border-collapse" "collapse"
    , style "font-size" "8pt"
    , style "width" "8em"
    ]


createCell : String -> List String -> Html ()
createCell color contents =
    td
        (cellStyles color ++ [ style "width" "" ])
        (contents |> List.map (\t -> text t))


cellFromOpcode : OpcodeData -> Html ()
cellFromOpcode opcode =
    td
        (cellStyles opcode.bgColor)
        [ text opcode.pneumonic
        , br [] []
        , span
            [ style "padding-right" "1em" ]
            [ text (String.fromInt opcode.size) ]
        , text opcode.timeTaken
        , br [] []
        , text opcode.flags
        ]


printOpcode : Opcode -> Html ()
printOpcode opcode =
    case opcode of
        Set data ->
            cellFromOpcode data

        Unset ->
            createCell "" [ "" ]


convertRow : Int -> List (Html ()) -> Html ()
convertRow index row =
    let
        rowLabel =
            createCell "#9f9f9f" [ String.fromInt index ]
    in
    tr [] (rowLabel :: row)


convertToTable : List (Html ()) -> Html ()
convertToTable rows =
    let
        headerRow =
            List.range 0 15
                |> List.map (\i -> createCell "#9f9f9f" [ String.fromInt i ])
                |> (\row -> tr [] (createCell "#9f9f9f" [ "" ] :: row))
    in
    table
        [ style "background-color" "#bfbfbf"
        , style "border-width" "1px"
        , style "font-family" "monospace"
        , style "line-height" "normal"
        , style "border-style" "solid"
        , style "border-color" "black"
        , style "border-collapse" "collapse"
        , style "text-align" "center"
        ]
        (headerRow :: rows)


main =
    -- Create all table coords
    -- Loop through table coords
    opcodes_dict
        |> List.map printOpcode
        |> split 16
        |> List.indexedMap convertRow
        |> convertToTable
