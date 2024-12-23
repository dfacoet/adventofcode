module Year2024.Day22 (part1, part2) where

import           Data.Bits (shiftL, shiftR, xor, (.&.))
import           Data.List (tails, transpose)
import qualified Data.Map  as Map

part1 :: String -> String
part1 = show . sum . map (evolveN 2000 . read) . lines

part2 :: String -> String
part2 input =
  show . maximum . Map.elems $
    foldl
      (Map.unionWith (+))
      Map.empty
      (map (countSeqs . read) (lines input))
  where
    countSeqs n =
      let digits = (map (`mod` 10) . take 2001 . iterate evolve) n
          diffs = zipWith (-) (tail digits) digits
       in foldl
            (\acc (k, v) -> Map.insertWith (\_ b -> b) k v acc)
            Map.empty
            (zip (windows 4 diffs) (drop 4 digits))

evolve :: Int -> Int
evolve = step3 . step2 . step1 -- TODO: simplify ops after composing
  where
    pruneConst = 16777216 - 1 -- 2^24 - 1
    step1 n = ((n `shiftL` 6) `xor` n) .&. pruneConst
    step2 n = ((n `shiftR` 5) `xor` n) .&. pruneConst
    step3 n = ((n `shiftL` 11) `xor` n) .&. pruneConst

evolveN :: Int -> Int -> Int
evolveN n x = iterate evolve x !! n

windows :: Int -> [a] -> [[a]]
windows n = takeWhile (\w -> length w == n) . transpose . take n . tails
