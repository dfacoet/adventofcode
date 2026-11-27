module Year2020.Day05 (part1, part2) where

import Data.List (sort)

part1 :: String -> String
part1 = show . maximum . map seatID . lines

part2 :: String -> String
part2 =
  show
    . (\t -> fst t + 1)
    . head
    . filter (\(x, y) -> y - x /= 1)
    . pairs
    . sort
    . map seatID
    . lines

seatID :: String -> Int
seatID s = 8 * row + column
  where
    (row, column) = parsePass s

mapTuple :: (a -> b) -> (a, a) -> (b, b)
mapTuple f (a1, a2) = (f a1, f a2)

parsePass :: String -> (Int, Int)
parsePass = mapTuple evaluate . splitAt 7

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
