module Yacht
    ( yacht
    , Category(..)
    ) where

import Data.Function (on)
import Data.List (group, maximumBy, sort, sortBy)

data Category
    = Ones
    | Twos
    | Threes
    | Fours
    | Fives
    | Sixes
    | FullHouse
    | FourOfAKind
    | LittleStraight
    | BigStraight
    | Choice
    | Yacht

ofAKind :: Ord a => Int -> [a] -> [a]
ofAKind n = concat . filter ((== n) . length) . group . sort

ofAKindTake :: Ord a => Int -> [a] -> [a]
ofAKindTake n xs =
    let xs' = (maximumBy (compare `on` length) . group . sort) xs
     in if n <= length xs'
            then take n xs'
            else []

yacht :: Category -> [Int] -> Int
yacht Ones = sum . filter (== 1)
yacht Twos = sum . filter (== 2)
yacht Threes = sum . filter (== 3)
yacht Fours = sum . filter (== 4)
yacht Fives = sum . filter (== 5)
yacht Sixes = sum . filter (== 6)
yacht FullHouse = check
  where
    check dice =
        let threes = ofAKind 3 dice
            twos = ofAKind 2 dice
         in if not (null threes) && not (null threes)
                then sum $ threes ++ twos
                else 0
yacht FourOfAKind = sum . ofAKindTake 4
yacht LittleStraight = check
  where
    check dice =
        if (== [1, 2, 3, 4, 5]) $ sort dice
            then 30
            else 0
yacht BigStraight = check
  where
    check dice =
        if (== [2, 3, 4, 5, 6]) $ sort dice
            then 30
            else 0
yacht Choice = sum
yacht Yacht = check
  where
    check dice =
        if not $ null $ ofAKind 5 dice
            then 50
            else 0
