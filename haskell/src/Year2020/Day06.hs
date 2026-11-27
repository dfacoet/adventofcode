module Year2020.Day06 (part1, part2) where

import Data.List.Split (splitOn)
import Data.Set (Set, fromList, intersection, union)

part1 :: String -> String
part1 = show . solve Data.Set.union . parseInput

part2 :: String -> String
part2 = show . solve Data.Set.intersection . parseInput

parseInput :: String -> [[String]]
parseInput = splitOn [""] . lines

countOp :: Ord a => (Set a -> Set a -> Set a) -> [[a]]  -> Int
countOp op = length . foldl1 op . map Data.Set.fromList

solve :: Ord a => (Set a -> Set a -> Set a) -> [[[a]]] -> Int
solve op = sum . map (countOp op)
