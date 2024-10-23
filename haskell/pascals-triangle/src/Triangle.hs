module Triangle
    ( rows
    ) where

coeff :: Int -> Int -> Integer
coeff n k
    | k == 0 = 1
    | n == k = 1
    | otherwise = coeff (n - 1) (k - 1) + coeff (n - 1) k

rows :: Int -> [[Integer]]
rows 0 = []
rows x = [1]: map (\x -> map (coeff x) [0 .. x]) [1 .. fromIntegral (x-1)]
