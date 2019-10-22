module OpcodeTable exposing (DisplayProperties, showAll, viewTable)

import Hex exposing (toHex)
import Html exposing (Html, br, span, table, td, text, tr)
import Html.Attributes exposing (style)
import Opcode exposing (Opcode(..), OpcodeData)
import OpcodesDict exposing (opcodes_dict)
import SupportedCodes exposing (isCoordSupported, supportedCodes)


type DisplayProperties
    = ShowAll


showAll : DisplayProperties
showAll =
    ShowAll


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


createCell : String -> List String -> Html msg
createCell color contents =
    td
        (cellStyles color ++ [ style "width" "" ])
        (contents |> List.map (\t -> text t))


cellFromOpcode : Bool -> OpcodeData -> Html msg
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


printOpcode : List DisplayProperties -> Int -> Opcode -> Html msg
printOpcode properties index opcode =
    let
        coord =
            ( modBy 16 index, index // 16 )

        shouldShowAll =
            List.member showAll properties
    in
    case opcode of
        Set data ->
            cellFromOpcode
                (if shouldShowAll then
                    True

                 else
                    isCoordSupported coord
                )
                data

        Unset ->
            createCell "" [ "" ]


convertRow : Int -> List (Html msg) -> Html msg
convertRow index row =
    let
        rowLabel =
            createCell "#9f9f9f" [ "0x" ++ toHex index ++ "X" ]
    in
    tr [] (rowLabel :: row)


convertToTable : List (Html msg) -> Html msg
convertToTable rows =
    let
        headerRow =
            List.range 0 15
                |> List.map (\i -> createCell "#9f9f9f" [ "0xX" ++ toHex i ])
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


viewTable : List DisplayProperties -> Html msg
viewTable properties =
    opcodes_dict
        |> List.indexedMap (printOpcode properties)
        |> split 16
        |> List.indexedMap convertRow
        |> convertToTable
