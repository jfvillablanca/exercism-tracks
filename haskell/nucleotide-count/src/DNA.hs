module DNA ( nucleotideCounts, Nucleotide(..) ) where

import Data.Map.Lazy

data Nucleotide = A | C | G | T
    deriving ( Eq, Ord, Show )

nucleotideCounts :: String -> Either String (Map Nucleotide Int)
nucleotideCounts = Prelude.foldl updateNucleotide (Right baseMap)
  where
    baseMap = fromList [(A, 0), (C, 0), (G, 0), (T, 0)]

    updateNucleotide :: Either String (Map Nucleotide Int) -> Char -> Either String (Map Nucleotide Int)
    updateNucleotide (Right counts) 'A' = Right (insertWith (+) A 1 counts)
    updateNucleotide (Right counts) 'C' = Right (insertWith (+) C 1 counts)
    updateNucleotide (Right counts) 'G' = Right (insertWith (+) G 1 counts)
    updateNucleotide (Right counts) 'T' = Right (insertWith (+) T 1 counts)
    updateNucleotide _ _ = Left "error"
