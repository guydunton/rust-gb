module Main exposing (main)

import Browser
import Html exposing (Html, div, h3, input, label, text)
import Html.Attributes exposing (for, id, type_)
import Html.Events exposing (onCheck)
import OpcodeTable exposing (showAll, showSupported, viewTable)
import OpcodesDict exposing (cb_opcodes_dict, opcodes_dict)
import SupportedCodes exposing (supportedCBCodes, supportedCodes)


type alias Model =
    { showSupported : Bool
    }


type Msg
    = ToggleSupported Bool


init : Model
init =
    { showSupported = False
    }


view : Model -> Html Msg
view model =
    let
        ( table, cbTable ) =
            if model.showSupported then
                ( viewTable opcodes_dict (showSupported supportedCodes)
                , viewTable cb_opcodes_dict (showSupported supportedCBCodes)
                )

            else
                ( viewTable opcodes_dict showAll
                , viewTable cb_opcodes_dict showAll
                )
    in
    div []
        [ input [ type_ "checkbox", onCheck ToggleSupported, id "show-supported" ] []
        , label [ for "show-supported" ] [ text "Show Supported" ]
        , table
        , h3 [] [ text "CB Opcodes" ]
        , div [] [ cbTable ]
        ]


update : Msg -> Model -> Model
update msg model =
    case msg of
        ToggleSupported checked ->
            { model | showSupported = checked }


main =
    Browser.sandbox
        { init = init
        , update = update
        , view = view
        }
