module RunLength
    ( decode
    , encode
    ) where

import Data.Char (isLetter, isSpace)
import Data.List (group)
import Data.List.Split (dropDelims, dropFinalBlank, onSublist, split)
import Data.Maybe (fromMaybe)
import Text.Read (readMaybe)

decode :: String -> String
decode encodedText = concatMap (uncurry replicate) $ generatePair encodedText

generatePair :: String -> [(Int, Char)]
generatePair xs =
    let parsePair :: String -> (Int, Char)
        parsePair pair =
            let count = fromMaybe 1 (readMaybe $ init pair :: Maybe Int)
             in (count, last pair)
     in map parsePair $ splitWords xs

splitWords :: String -> [String]
splitWords xs' =
    (split . dropDelims . dropFinalBlank . onSublist) "<>"
        $ foldl
              (\acc c ->
                   if isLetter c || isSpace c
                       then acc ++ [c] ++ "<>"
                       else acc ++ [c])
              ""
              xs'

encode :: String -> String
encode text =
    let createPairs :: String -> [(Int, Char)]
        createPairs text' = map (\xs -> (length xs, head xs)) $ group text'
        encodePair :: (Int, Char) -> String
        encodePair (1, ch) = [ch]
        encodePair (ct, ch) = show ct ++ [ch]
     in concatMap encodePair $ createPairs text
