module Year2019.Day04 (part1, part2) where

import           Data.List       (group)
import           Data.List.Split (splitOn)

part1 :: String -> String
part1 = show . part1' . parseInput

part2 :: String -> String
part2 = show . part2' . parseInput

parseInput :: String -> (Integer, Integer)
parseInput input = case splitOn "-" input of
  [a, b] -> (read a, read b)
  _      -> error "Invalid input"

part1' :: (Integer, Integer) -> Int
part1' (minP, maxP) = length $ filter isValidPassword1 [minP .. maxP]

isValidPassword1 :: Integer -> Bool
isValidPassword1 p = hasAdjacentDigits p && isIncreasing p

part2' :: (Integer, Integer) -> Int
part2' (minP, maxP) = length $ filter isValidPassword2 [minP .. maxP]

isValidPassword2 :: Integer -> Bool
isValidPassword2 p = hasTwoAdjacentDigits p && isIncreasing p

hasAdjacentDigits :: Integer -> Bool
hasAdjacentDigits p = any ((>= 2) . length) (group (show p))

isIncreasing :: Integer -> Bool
isIncreasing p = all (uncurry (<=)) $ zip (show p) (tail (show p))

hasTwoAdjacentDigits :: Integer -> Bool
hasTwoAdjacentDigits p = any ((== 2) . length) (group (show p))
