module WordCount ( wordCount ) where

import Data.Char ( toLower, isSpace, isAsciiLower, isNumber )

lowercaseString :: String -> String
lowercaseString = map toLower

isLowerOrNumber :: Char -> Bool
isLowerOrNumber x = isAsciiLower x || isNumber x

removePunctuation :: String -> String
removePunctuation [] = []
removePunctuation [ x ] = [ x | isLowerOrNumber x ]
removePunctuation ('\'' : xs)
    | last xs == '\'' = init xs
removePunctuation (x : y : xs)
    | x == '\'' = x : y : removePunctuation xs
    | isLowerOrNumber x = x : removePunctuation
        (y : xs)
    | otherwise = removePunctuation (y : xs)

-- This implementation is taken 
-- from the `Data.List.lines` function
splitter :: String -> [ String ]
splitter st = concatMap lines $ wordSplitter st
  where
    wordSplitter :: String -> [ String ]
    wordSplitter "" = []
    wordSplitter s = cons (case break (\c -> isSpace c || c == ',') s of
                               ( l, s' ) -> ( l
                                            , case s' of
                                                  [] -> []
                                                  _ : s'' -> wordSplitter s''
                                            ))
      where
        cons ~( h, t ) = h : t

wordCount :: String -> [ ( String, Int ) ]
wordCount phrase = wordCounter $ phraseToWordList phrase
  where
    phraseToWordList :: String -> [ String ]
    phraseToWordList p = filter (/= "") $ map removePunctuation
        (splitter $ lowercaseString p)

    wordCounter :: [ String ] -> [ ( String, Int ) ]
    wordCounter [] = []
    wordCounter [ x ] = [ ( x, 1 ) ]
    wordCounter (x : xs) = foldl
        (\acc wordFromList
         -> let headEnd = takeWhile (\( curWord, _ ) -> curWord /= wordFromList) acc
         in case dropWhile (\( curWord, _ ) -> curWord /= wordFromList) acc of
             [] -> ( wordFromList, 1 ) : acc
             (( _, oldCount ) : tailEnd)
                 -> headEnd ++ [ ( wordFromList, oldCount + 1 ) ] ++ tailEnd)
        [ ( x, 1 ) ] xs

