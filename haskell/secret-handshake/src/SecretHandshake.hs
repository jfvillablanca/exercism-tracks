module SecretHandshake
    ( handshake
    ) where

import Data.Bits ((.&.))

actionPairs :: [(Int, String)]
actionPairs =
    [(1, "wink"), (2, "double blink"), (4, "close your eyes"), (8, "jump")]

handshake :: Int -> [String]
handshake n =
    let actions =
            filter (/= "")
                $ map (\(bit, action) ->
                           if (n .&. bit) /= 0
                               then action
                               else "")
                      actionPairs
        reverseBit = 16
     in case n .&. reverseBit of
            0 -> actions
            _ -> reverse actions
