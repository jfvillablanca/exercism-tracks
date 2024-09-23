module SumOfMultiples
  ( sumOfMultiples
  ) where

import Prelude

import Data.Foldable (sum)
import Data.Array (concatMap, filter, nubByEq, (..))

sumOfMultiples :: Array Int -> Int -> Int
sumOfMultiples factors limit =
  sum
    $ nubByEq (==)
    $
      concatMap
        ( \factor -> filter
            ( \possibleMultiple -> factor <= limit &&
                possibleMultiple `mod` factor == 0
            )
            (factor .. (limit - 1))
        )
        factors
