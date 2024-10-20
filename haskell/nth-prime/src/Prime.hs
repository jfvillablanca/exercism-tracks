module Prime
    ( nth
    ) where

import Control.Monad (guard)

nth :: Int -> Maybe Integer
nth 0 = Nothing
nth n = Just ((last . take n) (2 : filter isPrime [3,5 ..]))

isPrime :: Integer -> Bool
isPrime n =
    foldr (\p r -> p * p > n || ((n `rem` p) /= 0 && r)) True (2 : [3,5 ..])
