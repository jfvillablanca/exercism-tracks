module DND
    ( Character(..)
    , ability
    , modifier
    , character
    ) where

import Test.QuickCheck (Arbitrary(arbitrary), Gen, choose)

data Character = Character
    { strength :: Int
    , dexterity :: Int
    , constitution :: Int
    , intelligence :: Int
    , wisdom :: Int
    , charisma :: Int
    , hitpoints :: Int
    } deriving (Show, Eq)

modifier :: Int -> Int
modifier x = (x - 10) `div` 2

ability :: Gen Int
ability = choose (3, 18)

instance Arbitrary Character where
    arbitrary = character

character :: Gen Character
character = do
    str <- ability
    dex <- ability
    con <- ability
    int <- ability
    wis <- ability
    cha <- ability
    return
        (Character
             { strength = str
             , dexterity = dex
             , constitution = con
             , intelligence = int
             , wisdom = wis
             , charisma = cha
             , hitpoints = 10 + modifier con
             })
