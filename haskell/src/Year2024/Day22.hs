module Year2024.Day22 (part1, part2) where

import Data.Bits (xor)

part1 :: String -> String
part1 input = show $ sum $ map (evolveN 2000) $ parseInput input

part2 :: String -> String
part2 = error "Part2 not implemented"

pruneAt :: Integer
pruneAt = 16777216 -- 2^24

parseInput :: String -> [Integer]
parseInput s = map read (lines s)

step1 :: Integer -> Integer
step1 n = ((n * 64) `xor` n) `mod` pruneAt

step2 :: Integer -> Integer
step2 n = ((n `div` 32) `xor` n) `mod` pruneAt

step3 :: Integer -> Integer
step3 n = ((n * 2048) `xor` n) `mod` pruneAt

evolve :: Integer -> Integer
evolve = step3 . step2 . step1

evolveN :: Int -> Integer -> Integer
evolveN n x = iterate evolve x !! n
