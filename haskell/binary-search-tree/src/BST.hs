module BST
    ( BST
    , bstLeft
    , bstRight
    , bstValue
    , empty
    , fromList
    , insert
    , singleton
    , toList
    ) where

data BST a
    = Leaf
    | Node (BST a) a (BST a)
    deriving (Eq, Show)

bstLeft :: BST a -> Maybe (BST a)
bstLeft Leaf = Nothing
bstLeft (Node left _ _) = Just left

bstRight :: BST a -> Maybe (BST a)
bstRight Leaf = Nothing
bstRight (Node _ _ right) = Just right

bstValue :: BST a -> Maybe a
bstValue (Node _ a _) = Just a
bstValue _ = Nothing

empty :: BST a
empty = Leaf

fromList :: Ord a => [a] -> BST a
fromList = foldl (flip insert) empty

insert :: Ord a => a -> BST a -> BST a
insert x Leaf = singleton x
insert x (Node left a right)
    | x > a = Node left a (insert x right)
    | otherwise = Node (insert x left) a right

singleton :: a -> BST a
singleton x = Node Leaf x Leaf

toList :: BST a -> [a]
toList Leaf = []
toList (Node left a right) = toList left ++ [a] ++ toList right
