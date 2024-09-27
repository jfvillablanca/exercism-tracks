module CollatzConjecture
    ( collatz
    ) where

collatz :: Integer -> Maybe Integer
collatz n
    | n <= 0 = Nothing
    | otherwise = Just (getCount 0 n)
  where
    getCount :: Integer -> Integer -> Integer
    getCount ct 1 = ct
    getCount ct x
        | even x = getCount (ct + 1) (x `div` 2)
        | otherwise = getCount (ct + 1) ((x * 3) + 1)
