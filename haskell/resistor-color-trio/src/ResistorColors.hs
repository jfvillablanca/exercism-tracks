module ResistorColors
    ( Color(..)
    , Resistor(..)
    , label
    , ohms
    ) where

import Data.Text (Text, pack)

data Color
    = Black
    | Brown
    | Red
    | Orange
    | Yellow
    | Green
    | Blue
    | Violet
    | Grey
    | White
    deriving (Show, Enum, Bounded)

newtype Resistor = Resistor
    { bands :: (Color, Color, Color)
    } deriving (Show)

label :: Resistor -> Text
label resistor =
    let strLabel :: Int -> String
        strLabel value'
            | value' < 1000 = show value' ++ " ohms"
            | value' < 10 ^ 6 = show (value' `div` 10 ^ 3) ++ " kiloohms"
            | value' < 10 ^ 9 = show (value' `div` 10 ^ 6) ++ " megaohms"
            | otherwise = show (value' `div` 10 ^ 9) ++ " gigaohms"
     in pack $ strLabel $ ohms resistor

ohms :: Resistor -> Int
ohms (Resistor (a, b, c)) = (fromEnum a * 10 + fromEnum b) * (10 ^ fromEnum c)
