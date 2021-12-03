main :: IO ()
main = do
    inp <- getContents
    let ns = map (read :: String -> Int) (lines inp)
    task ns
    task $ map sum (window 3 ns)

task :: [Int] -> IO ()
task ns =
    let events = zipWith (\x y -> if x < y then 1 else 0) ns (tail ns) in
    print (sum events)

window :: Int -> [a] -> [[a]]
window n xs | length xs >= n =
    let y = take n xs
        rest = tail xs
    in y : window n rest
window _ _ = []
