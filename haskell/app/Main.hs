module Main (main) where

import qualified Data.Map as Map
import System.Environment (getArgs)
import Text.Printf (printf)
import qualified Year2015.Day01

main :: IO ()
main = do
  args <- getArgs
  case args of
    [yearStr, dayStr] -> case (reads yearStr, reads dayStr) of
      ((year, "") : _, (day, "") : _) -> runSolution year day
      _ -> usageError
    _ -> usageError

usageError :: IO ()
usageError = putStrLn "Usage: haskell-exe <year> <day>"

runSolution :: Integer -> Integer -> IO ()
runSolution year day = do
  input <- readFile $ printf "input/y%dd%02d.txt" year day
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
    [ ((2015, 1), (Year2015.Day01.part1, Year2015.Day01.part2))
    ]
