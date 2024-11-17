module Year2019.Day01 (part1, part2) where

part1 :: String -> String
part1 = show . sum . map fuel . parseInput

part2 :: String -> String
part2 = show . sum . map totalFuel . parseInput

parseInput :: String -> [Int]
parseInput = map read . lines

fuel :: Int -> Int
fuel mass = div mass 3 - 2

totalFuel :: Int -> Int
totalFuel mass = sum (takeWhile (> 0) (tail (iterate fuel mass)))
