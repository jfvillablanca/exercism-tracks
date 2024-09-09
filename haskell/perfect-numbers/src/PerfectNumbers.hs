module PerfectNumbers ( classify, Classification(..) ) where

data Classification = Deficient | Perfect | Abundant
    deriving ( Eq, Show )

classify :: Int -> Maybe Classification
classify num
    | num <= 0 = Nothing
    | aliquotSum num == num = Just Perfect
    | aliquotSum num > num = Just Abundant
    | aliquotSum num < num = Just Deficient
    | otherwise = Nothing
  where
    aliquotSum :: Int -> Int
    aliquotSum someNum
        = sum [ x | x <- [ 1 .. someNum - 1 ], mod someNum x == 0 ]
