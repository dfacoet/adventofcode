module Main (main) where

import qualified Data.Map           as Map
import           System.Environment (getArgs)
import           Text.Printf        (printf)
import qualified Year2015.Day01
import qualified Year2019.Day01
import qualified Year2019.Day04
import qualified Year2019.Day08
import qualified Year2024.Day01
import qualified Year2024.Day02
import qualified Year2024.Day04

main :: IO ()
main = do
  args <- getArgs
  case args of
    [yearStr, dayStr] -> case (reads yearStr, reads dayStr) of
      ((year, "") : _, (day, "") : _) -> runSolution year day Nothing
      _                               -> usageError
    [yearStr, dayStr, "--input", inputPath] -> case (reads yearStr, reads dayStr) of
      ((year, "") : _, (day, "") : _) -> runSolution year day (Just inputPath)
      _                               -> usageError
    _ -> usageError

usageError :: IO ()
usageError = putStrLn "Usage: haskell-exe <year> <day> [optional: --input <input_file_path>]"

runSolution :: Integer -> Integer -> Maybe String -> IO ()
runSolution year day path = do
  input <-
    readFile
      ( case path of
          Just p  -> p
          Nothing -> printf "input/y%dd%02d.txt" year day
      )
  putStrLn $ printf "year %d day %02d" year day
  putStrLn "================"
  case Map.lookup (year, day) solutionMap of
    Just (part1, part2) -> do
      putStrLn $ "part1: " ++ part1 input
      putStrLn $ "part2: " ++ part2 input
    Nothing -> putStrLn $ printf "No solution found for y%dd%02d" year day

type SolutionFn = String -> String

solutionMap :: Map.Map (Integer, Integer) (SolutionFn, SolutionFn)
solutionMap =
  Map.fromList
    [ ((2015, 1), (Year2015.Day01.part1, Year2015.Day01.part2)),
      ((2019, 1), (Year2019.Day01.part1, Year2019.Day01.part2)),
      ((2019, 4), (Year2019.Day04.part1, Year2019.Day04.part2)),
      ((2019, 8), (Year2019.Day08.part1, Year2019.Day08.part2)),
      ((2024, 1), (Year2024.Day01.part1, Year2024.Day01.part2)),
      ((2024, 2), (Year2024.Day02.part1, Year2024.Day02.part2)),
      ((2024, 4), (Year2024.Day04.part1, Year2024.Day04.part2))
    ]
