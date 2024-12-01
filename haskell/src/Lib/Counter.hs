module Lib.Counter (Counter, count) where

import           Data.Map (Map, empty, insertWith)

type Counter k v = Map k v


count :: (Ord k, Num v) => [k] -> Counter k v
count = foldl (\acc key -> insertWith (+) key 1 acc) empty
