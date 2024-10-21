module StateOfTicTacToe
    ( gameState
    , GameState(..)
    ) where

import Data.List (transpose)

data Piece
    = X
    | O
    deriving (Eq, Show)

convertToPiece :: Char -> Maybe Piece
convertToPiece 'X' = Just X
convertToPiece 'O' = Just O
convertToPiece _ = Nothing

data GameState
    = WinX
    | WinO
    | Draw
    | Ongoing
    | Impossible
    deriving (Eq, Show)

-- checkDiagonal :: [String] -> Piece -> Bool
checkDiagonal [[a, _, _], [_, b, _], [_, _, c]] piece =
    a == b && b == c && convertToPiece a == Just piece
checkDiagonal _ _ = False

checkLine :: [String] -> Piece -> Bool
checkLine board piece =
    any ((== 3) . length . filter (\x -> convertToPiece x == Just piece)) board

checkWin :: [String] -> Piece -> Bool
checkWin board piece =
    checkLine board piece
        || checkLine (transpose board) piece
        || checkDiagonal board piece
        || checkDiagonal (reverse board) piece

countPieces :: [String] -> Piece -> Int
countPieces board piece =
    length $ filter (\x -> convertToPiece x == Just piece) $ concat board

checkOngoing :: [String] -> Bool
checkOngoing board = 9 /= length (filter (/= ' ') (concat board))

checkImpossibility :: [String] -> Bool
checkImpossibility board = continuedPlayAfterWin board || wrongTurnOrder board
  where
    continuedPlayAfterWin board' = checkWin board' X && checkWin board' O
    wrongTurnOrder board' =
        countPieces board' O > countPieces board' X
            || countPieces board' O < countPieces board' X - 1

gameState :: [String] -> GameState
gameState board
    | checkImpossibility board = Impossible
    | checkWin board X = WinX
    | checkWin board O = WinO
    | checkOngoing board = Ongoing
    | otherwise = Draw
