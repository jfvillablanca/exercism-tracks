module Triangle
  ( Triangle(Equilateral, Isosceles, Scalene)
  , triangleKind
  ) where

import Prelude

import Data.Array (any)
import Data.Either (Either(..))

data Triangle
  = Equilateral
  | Isosceles
  | Scalene

derive instance eqTriangle :: Eq Triangle

instance Show Triangle where
  show Equilateral = "Equilateral"
  show Isosceles = "Isosceles"
  show Scalene = "Scalene"

triangleKind :: Int -> Int -> Int -> Either String Triangle
triangleKind x y z
  | any (_ <= 0) [ x, y, z ] = Left "Invalid lengths"
  | not (x + y >= z && z + y >= x && z + x >= y) = Left "Violates inequality"
  | x == y && y == z = Right Equilateral
  | x == y || y == z || x == z = Right Isosceles
  | otherwise = Right Scalene
