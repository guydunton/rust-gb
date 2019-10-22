module Main exposing (main)

import Html exposing (Html, br, p, span, table, td, text, tr)
import Html.Attributes exposing (style)
import Opcode exposing (Opcode(..), OpcodeData)
import OpcodesDict exposing (opcodes_dict)
import SupportedCodes exposing (isCoordSupported, supportedCodes)


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
    , style "font-size" "6pt"
    , style "width" "8em"
    ]


createCell : String -> List String -> Html ()
createCell color contents =
    td
        (cellStyles color ++ [ style "width" "" ])
        (contents |> List.map (\t -> text t))


cellFromOpcode : Bool -> OpcodeData -> Html ()
cellFromOpcode isSupported opcode =
    td
        (cellStyles
            (if isSupported then
                opcode.bgColor

             else
                "#fbfbfb"
            )
        )
        [ text opcode.pneumonic
        , br [] []
        , span
            [ style "padding-right" "1em" ]
            [ text (String.fromInt opcode.size) ]
        , text opcode.timeTaken
        , br [] []
        , text opcode.flags
        ]


printOpcode : Int -> Opcode -> Html ()
printOpcode index opcode =
    let
        coord =
            ( modBy 16 index, index // 16 )
    in
    case opcode of
        Set data ->
            cellFromOpcode (isCoordSupported coord) data

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
        |> List.indexedMap printOpcode
        |> split 16
        |> List.indexedMap convertRow
        |> convertToTable
