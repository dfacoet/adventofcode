module Year2020.Day05 (part1, part2) where

import Control.Monad (msum)
import Data.List (sort)
import Data.Maybe (fromJust)

part1 :: String -> String
part1 = show . maximum . map seatID . lines

part2 :: String -> String
part2 =
  show
    . fromJust
    . findGap
    . map seatID
    . lines

seatID :: String -> Int
seatID s = 8 * row + column
  where
    (row, column) = mapTuple evaluate $ splitAt 7 s

mapTuple :: (a -> b) -> (a, a) -> (b, b)
mapTuple f (a1, a2) = (f a1, f a2)

evaluate :: String -> Int
evaluate s = sum $ zipWith (curry evalDigit) [0 ..] (reverse s)
  where
    evalDigit :: (Int, Char) -> Int
    evalDigit (k, c) = case c of
      x | x `elem` "FL" -> 0
      x | x `elem` "BR" -> 2 ^ k
      _ -> error "Invalid input"

pairs :: [a] -> [(a, a)]
pairs (x1 : x2 : xs) = (x1, x2) : pairs (x2 : xs)
pairs _ = []

findGap :: [Int] -> Maybe Int
findGap = msum . map f . pairs . sort
  where
    f (x, y) = case y - x of
      2 -> Just (x + 1)
      _ -> Nothing
