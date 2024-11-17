module Year2019.Day08 (part1, part2) where

import Data.Foldable (minimumBy)
import Data.List (dropWhileEnd)
import qualified Data.Map as Map
import Data.Ord (comparing)

part1 :: String -> String
part1 s = show $ score $ minimumBy nZerosComp $ map countOccurrences $ layers $ dropWhileEnd (== '\n') s
  where
    nZerosComp = comparing (Map.findWithDefault 0 '0')
    score counts = Map.findWithDefault 0 '1' counts * Map.findWithDefault 0 '2' counts

part2 :: String -> String
-- returns a string of ' ' and '█' that, when printed, shows the solution with each letter
-- represented as 6x4 block with a column of spaces in between.
-- TODO: convert to the actual solution string to allow automated testing
part2 s = unlines $ reshape layerWidth $ map getFirstPixel $ transpose $ layers s
  where
    getFirstPixel sec = case head $ filter (/= '2') sec of
      '0' -> ' '
      '1' -> '█'

layerHeight :: Int
layerHeight = 6

layerWidth :: Int
layerWidth = 25

reshape :: Int -> [a] -> [[a]]
reshape _ [] = []
reshape n xs
  | n > length xs = error "length of list not divisible by n"
  | otherwise = take n xs : reshape n (drop n xs)

layers :: [a] -> [[a]]
layers = reshape (layerHeight * layerWidth)

countOccurrences :: (Ord a) => [a] -> Map.Map a Int
countOccurrences = foldr (\x acc -> Map.insertWith (+) x 1 acc) Map.empty

transpose :: [[a]] -> [[a]]
transpose ([] : _) = []
transpose rows = map head rows : transpose (map tail rows)
