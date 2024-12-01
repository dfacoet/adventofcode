module Year2024.Day01 (part1, part2) where

import           Data.Counter (count)
import           Data.List    (sort)
import           Data.Map     (findWithDefault)

part1 :: String -> String
part1 input =
  let (leftList, rightList) = parseInput input
   in show $
        sum $
          zipWith
            (\x y -> abs (x - y))
            (sort leftList)
            (sort rightList)

part2 :: String -> String
part2 input = show $ sum $ map (\x -> x * findWithDefault 0 x rightCounts) leftList
  where
    (leftList, rightList) = parseInput input
    rightCounts = count rightList


parseInput :: String -> ([Integer], [Integer])
parseInput input =
  foldr
    ( \s (xs, ys) -> case words s of
        [x, y] -> (read x : xs, read y : ys)
        _      -> error "invalid line"
    )
    ([], [])
    (lines input)
