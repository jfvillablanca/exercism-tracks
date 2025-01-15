module ProteinTranslation
    ( proteins
    ) where

splitEmUp :: String -> [String]
splitEmUp [] = []
splitEmUp xs =
    let (a, as) = splitAt 3 xs
     in a : splitEmUp as

parseUntilStop :: [String] -> [String]
parseUntilStop = takeWhile (\x -> x /= "UAA" && x /= "UAG" && x /= "UGA")

translate "AUG" = "Methionine"
translate "UUU" = "Phenylalanine"
translate "UUC" = "Phenylalanine"
translate "UUA" = "Leucine"
translate "UUG" = "Leucine"
translate "UCU" = "Serine"
translate "UCC" = "Serine"
translate "UCA" = "Serine"
translate "UCG" = "Serine"
translate "UAU" = "Tyrosine"
translate "UAC" = "Tyrosine"
translate "UGU" = "Cysteine"
translate "UGC" = "Cysteine"
translate "UGG" = "Tryptophan"

proteins :: String -> Maybe [String]
proteins = Just . map translate . parseUntilStop . splitEmUp
