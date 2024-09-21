module Hamming
  ( distance
  ) where

import Prelude

import Data.Array (filter, length, zipWith) as A
import Data.Maybe (Maybe(..))
import Data.String (length) as S
import Data.String.CodeUnits (toCharArray)

zipWithString :: forall (a :: Type). (Char -> Char -> a) -> String -> String -> Array a
zipWithString f s1 s2 = A.zipWith f (toCharArray s1) (toCharArray s2)

distance :: String -> String -> Maybe Int
distance xs ys
  | S.length xs == S.length ys = Just (A.length $ A.filter identity $ zipWithString (/=) xs ys)
  | otherwise = Nothing
