module SumOfMultiples ( sumOfMultiples ) where

import Data.List ( nub )

sumOfMultiples :: [ Integer ] -> Integer -> Integer
sumOfMultiples factors limit
    = sum $ nub [ x * y | x <- factors, y <- [ 1 .. limit ], x * y < limit ]
