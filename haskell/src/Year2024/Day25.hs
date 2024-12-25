module Year2024.Day25 (part1, part2) where

import           Data.List       (transpose)
import           Data.List.Split (splitOn)

part1 :: String -> String
part1 input =
  let (keys, locks) = parseInput input
   in (show . length) $ [() | k <- keys, l <- locks, fits k l]

part2 :: String -> String
part2 = error "Part2 not implemented"

parseInput :: String -> ([[Int]], [[Int]])
parseInput input =
  let blocks = map lines (splitOn "\n\n" input)
      countHash block = map (length . filter (== '#')) (transpose block)
   in foldl
        ( \(keys, locks) block ->
            let cs = [countHash block]
             in case head block of
                  "#####" -> (keys, locks ++ cs)
                  "....." -> (keys ++ cs, locks)
                  _       -> error "Invalid block"
        )
        ([], [])
        blocks

fits :: [Int] -> [Int] -> Bool
fits key lock = all (<= 7) (zipWith (+) key lock)
