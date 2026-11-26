module Lib.Combinations (pairs, triples) where

pairs :: [a] -> [(a, a)]
pairs (x:ys) = [(x, y) | y <- ys] ++ pairs ys
pairs [] = []

triples :: [a] -> [(a, a, a)]
triples (x:ys) = [(x, y, z) | (y, z) <- pairs ys] ++ triples ys
triples [] = []
