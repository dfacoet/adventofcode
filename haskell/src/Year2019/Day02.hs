module Year2019.Day02 (part1, part2) where

import Data.List.Split (splitOn)

part1 :: String -> String
part1 input = let ns = parseInput input
  in show $ head (runProgram 0 ns)

-- 116 is too low

runProgram :: Int -> [Int] -> [Int]
runProgram i program = case program !! i of
    99 -> program
    1 -> runProgram (i+4) $ replaceElement (program !! (i+3)) (program !! (i+1) + program !! (i+2)) program
    2 -> runProgram (i+4) $ replaceElement (program !! (i+3)) (program !! (i+1) * program !! (i+2)) program
    _ -> error ("Invalid opcode " ++ show (program !! i) ++ " at " ++ show i)

replaceElement :: Int -> Int -> [Int] -> [Int]
replaceElement i x xs = let (ys, zs) = splitAt i xs in
    ys ++ [x] ++ drop 1 zs

part2 :: String -> String
part2 = error "Part2 not implemented"

parseInput :: String -> [Int]
parseInput = map read . splitOn ","
