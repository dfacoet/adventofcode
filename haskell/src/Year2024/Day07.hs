module Year2024.Day07 (part1, part2) where

part1 :: String -> String
part1 input = show . sum . map fst . filter isPossible $ parseInput input

part2 :: String -> String
part2 input = show . sum . map fst . filter isPossibleWithConcat $ parseInput input

parseInput :: String -> [(Integer, [Integer])]
parseInput =
  map
    ( \l ->
        let xs = words l
         in (read (init (head xs)), map read (tail xs))
    )
    . lines

isPossible :: (Integer, [Integer]) -> Bool
isPossible (test, xs) = test `elem` allEvals xs

allEvals :: [Integer] -> [Integer]
allEvals (x : xs) = foldl (\rs y -> map (+ y) rs ++ map (* y) rs) [x] xs
allEvals _ = error "expressions need at least two elements"

isPossibleWithConcat :: (Integer, [Integer]) -> Bool
isPossibleWithConcat (test, xs) = test `elem` allEvalsWithConcat xs

allEvalsWithConcat :: [Integer] -> [Integer]
allEvalsWithConcat (x : xs) = foldl (\rs y -> map (+ y) rs ++ map (* y) rs ++ map (`intConcat` y) rs) [x] xs
  where
    intConcat y z = read (show y ++ show z)
allEvalsWithConcat _ = error "expressions need at least two elements"
