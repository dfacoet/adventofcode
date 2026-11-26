module Year2020.Day01 (part1, part2) where

part1 :: String -> String
part1 input = show (x * y)
    where
        (x, y) = head
            [(a, b)
            | (a, b) <- pairs (parseInput input),
            a + b == 2020]

part2 :: String -> String
part2 input = show (x * y * z)
    where
        (x, y, z) = head
            [(a, b, c)
            | (a, b, c) <- triples (parseInput input),
            a + b + c == 2020]

parseInput :: String -> [Int]
parseInput = map read . lines

pairs :: [Int] -> [(Int, Int)]
pairs (x:ys) = [(x, y) | y <- ys] ++ pairs ys
pairs [] = []

triples :: [Int] -> [(Int, Int, Int)]
triples (x:ys) = [(x, y, z) | (y, z) <- pairs ys] ++ triples ys
triples [] = []
