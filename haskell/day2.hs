data Command = Up Int | Down Int | Forward Int deriving (Eq, Show)
type Pos1 = (Int, Int)
type Pos2 = (Int, Int, Int)

main :: IO ()
main = do
    inp <- getContents
    let cs = map parseCommand (lines inp)
    let (x1, y1) = foldr move1 (0,0) cs
    let (x2, y2, _) = foldr move2 (0,0,0) cs
    print (x1 * y1, x2 * y2)

parseCommand :: String -> Command
parseCommand str =
    let (c, n) = break (== ' ') str
        cmd = case c of { "up" -> Up; "down" -> Down; "forward" -> Forward } in
    cmd (read n)

move1 :: Command -> Pos1 -> Pos1
move1 c (x, y) = case c of
    Up n -> (x, y - n)
    Down n -> (x, y + n)
    Forward n -> (x + n, y)

move2 :: Command -> Pos2 -> Pos2
move2 c (x, y, aim) = case c of
    Up n -> (x, y, aim - n)
    Down n -> (x, y, aim + n)
    Forward n -> (x + n, y + n * aim, aim)
