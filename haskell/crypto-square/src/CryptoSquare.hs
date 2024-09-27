module CryptoSquare
    ( encode
    ) where

import Control.Monad (guard)
import Data.Char (isAlphaNum, toLower)
import Data.List (transpose)

normalize :: String -> String
normalize xs = filter isAlphaNum $ map toLower xs

findDimensions :: Int -> (Int, Int)
findDimensions len =
    head $ do
        r <- [1 .. len]
        c <- [r .. len]
        guard $ r * c >= len && c - r <= 1
        pure (r, c)

squareText :: String -> Int -> [String]
squareText texto len = reverse $ splitter (texto, [])
  where
    splitter params =
        case params of
            ("", acc) -> acc
            (text, acc) ->
                let (word, rest) = splitAt len text
                    wordLen = length word
                    paddedWord = word ++ replicate (len - wordLen) ' '
                 in splitter (rest, paddedWord : acc)

encode :: String -> String
encode xs =
    let normalizedCorpus = normalize xs
        cols = snd $ findDimensions $ length normalizedCorpus
        squaredCipher = unwords $ transpose $ squareText normalizedCorpus cols
     in squaredCipher
