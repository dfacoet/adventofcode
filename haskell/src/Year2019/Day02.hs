module Year2019.Day02 (part1, part2) where

import Data.List.Split (splitOn)

part1 :: String -> String
part1 input = show $ runProgram (parseInput input) 12 2

part2 :: String -> String
part2 input = show (100 * noun + verb)
  where
    p = parseInput input
    (noun, verb) =
      head
        [ (n, v)
          | n <- [0 .. 99],
            v <- [0 .. 99],
            runProgram p n v == 19690720
        ]

parseInput :: String -> [Int]
parseInput = map read . splitOn ","

runProgram :: [Int] -> Int -> Int -> Int
runProgram xs noun verb = head $ execute 0 initial
  where
    initial = [head xs] ++ [noun, verb] ++ drop 3 xs

execute :: Int -> [Int] -> [Int]
execute k xs = case xs !! k of -- unsafe indexing
  1 -> execute (k + 4) $ replacek (x1 + x2) xs
  2 -> execute (k + 4) $ replacek (x1 * x2) xs
  99 -> xs
  _ -> error "Invalid opcode"
  where
    replacek = replace (xs !! (k + 3))
    x1 = xs !! (xs !! (k + 1))
    x2 = xs !! (xs !! (k + 2))

replace :: Int -> Int -> [Int] -> [Int]
replace k n xs = a ++ [n] ++ drop 1 b
  where
    (a, b) = splitAt k xs
