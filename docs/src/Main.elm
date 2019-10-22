module Main exposing (main)

import Browser
import Html exposing (Html, div, input, label, text)
import Html.Attributes exposing (for, id, type_)
import Html.Events exposing (onCheck)
import OpcodeTable exposing (showAll, viewTable)


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
        table =
            viewTable
                (if not model.showSupported then
                    [ showAll ]

                 else
                    []
                )
    in
    div []
        [ table
        , input [ type_ "checkbox", onCheck ToggleSupported, id "show-supported" ] []
        , label [ for "show-supported" ] [ text "Show Supported" ]
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
