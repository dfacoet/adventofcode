module Year2024.Day02 (part1, part2) where

part1 :: String -> String
part1 input = show $ length $ filter isValid (parseInput input)

part2 :: String -> String
part2 input = show $ length $ filter (any isValid . dropEach) (parseInput input)

parseInput :: String -> [[Integer]]
parseInput input = map (map read . words) (lines input)

diff :: (Num a) => [a] -> [a]
diff xs = zipWith (-) xs (tail xs)

isValid :: (Eq a, Num a) => [a] -> Bool
isValid levels =
  let diffs = diff levels
   in all (`elem` [1, 2, 3]) diffs || all (`elem` [-1, -2, -3]) diffs

dropEach :: [a] -> [[a]]
dropEach xs = [take k xs ++ drop (k + 1) xs | k <- [0 .. length xs - 1]]
