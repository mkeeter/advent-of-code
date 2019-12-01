import qualified Data.Map as M
import Data.List.Split
import Data.Maybe
import Linear.V2
import Linear.Matrix

data Cell = Clean | Weakened | Infected | Flagged deriving (Show, Eq)
type Grid = M.Map (V2 Int) Cell

readLine :: Int -> Grid -> String -> Grid
readLine i g s = foldl (\g' k -> M.insert (V2 i k) Infected g')
                        g [k | (k,c) <- zip [0..] s, c == '#']

load :: String -> Grid
load s = foldl (\g' (i,line) -> readLine i g' line) M.empty
         $ zip [0..] $ lines s

rotateL = V2 (V2 0 (-1)) (V2 1 0)
rotateR = V2 (V2 0 1) (V2 (-1) 0)
continue = V2 (V2 1 0) (V2 0 1)
reverseDir = V2 (V2 (-1) 0) (V2 0 (-1))

get :: V2 Int -> Grid -> Cell
get pos g = fromMaybe Clean $ M.lookup pos g

step :: (Grid, V2 Int, V2 Int) -> (Grid, V2 Int, V2 Int)
step (g, pos, dir) =
  let infected = get pos g
      m = case infected of
              Clean -> rotateL
              Weakened -> continue
              Infected -> rotateR
              Flagged -> reverseDir
      dir' = m !* dir
      g' = case infected of
              Clean -> M.insert pos Weakened g
              Weakened -> M.insert pos Infected g
              Infected -> M.insert pos Flagged g
              Flagged -> M.delete pos g
      pos' = pos + dir'
    in (g', pos', dir')

main = do
  d <- readFile "input22.txt"
  let g = load d
  let size = length $ head $ lines d
  let center = size `quot` 2
  print $ length
         $ filter (\(g, p, _) -> Weakened == get p g)
         $ take 10000000
         $ iterate step (g, V2 center center, V2 (-1) 0)
