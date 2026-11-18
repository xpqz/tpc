program StringPosLoop;
var s: String; n, p, at: Integer;
begin
  s := 'one fish two fish red fish blue fish';
  n := 0; at := 0;
  while Pos('fish', Copy(s, at + 1, 255)) > 0 do begin
    p := Pos('fish', Copy(s, at + 1, 255));
    at := at + p + 3;
    n := n + 1;
    Write(at, ' ')
  end;
  WriteLn(n)
end.
