module IsbnVerifier
    ( isbn
    ) where

import Data.Char (digitToInt, ord)

compute :: [Int] -> Bool
compute xs = (== 0) $ (`mod` 11) $ sum $ zipWith (*) (reverse [1 .. 10]) xs

verify :: [Char] -> Bool
verify xs = all (`elem` "0123456789a") xs && compute (map digitToInt xs)

isbn :: String -> Bool
isbn [d1, '-', d2, d3, d4, '-', d5, d6, d7, d8, d9, '-', 'X'] =
    verify [d1, d2, d3, d4, d5, d6, d7, d8, d9, 'a']
isbn [d1, '-', d2, d3, d4, '-', d5, d6, d7, d8, d9, '-', d10] =
    verify [d1, d2, d3, d4, d5, d6, d7, d8, d9, d10]
isbn [d1, d2, d3, d4, d5, d6, d7, d8, d9, d10] =
    verify [d1, d2, d3, d4, d5, d6, d7, d8, d9, d10]
isbn _ = False
