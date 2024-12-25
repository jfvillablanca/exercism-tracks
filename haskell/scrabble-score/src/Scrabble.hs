module Scrabble
    ( scoreLetter
    , scoreWord
    ) where

import Data.Char (toLower)

scoreLetter :: Char -> Integer
scoreLetter letter
    | toLower letter `elem` "aeioulnrst" = 1
    | toLower letter `elem` "dg" = 2
    | toLower letter `elem` "bcmp" = 3
    | toLower letter `elem` "fhvwy" = 4
    | toLower letter `elem` "k" = 5
    | toLower letter `elem` "jx" = 8
    | toLower letter `elem` "qz" = 10
    | otherwise = 0

scoreWord :: String -> Integer
scoreWord = sum . map scoreLetter
