module Year2024.Day07 (part1, part2) where

part1 :: String -> String
part1 = solve [(+), (*)]

part2 :: String -> String
part2 = solve [(+), (*), intConcat]
  where
    intConcat y z = read (show y ++ show z)

solve :: [Integer -> Integer -> Integer] -> String -> String
solve ops = show . sum . map fst . filter (isPossible ops) . parseInput

parseInput :: String -> [(Integer, [Integer])]
parseInput =
  map
    ( \l ->
        let xs = words l
         in (read (init (head xs)), map read (tail xs))
    )
    . lines

isPossible :: [Integer -> Integer -> Integer] -> (Integer, [Integer]) -> Bool
isPossible ops (test, xs) = test `elem` allEvals ops xs

allEvals :: [Integer -> Integer -> Integer] -> [Integer] -> [Integer]
allEvals ops (x : xs) = foldl applyOps [x] xs
  where
    -- Applicative equivalent of
    -- concatMap (\op -> map (`op` y) rs) ops
    applyOps rs y = liftA2 ($ y) (flip <$> ops) rs
allEvals _ _ = error "Expressions must have at least two terms"
