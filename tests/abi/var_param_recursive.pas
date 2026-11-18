program VarParamRecursive;
{ var parameters in recursion: each level passes its own var parameter down and also a
  fresh local }
procedure Collatz(n: LongInt; var steps: Integer; var peak: LongInt);
begin
  if n > peak then peak := n;
  if n <> 1 then begin
    steps := steps + 1;
    if Odd(n) then Collatz(3 * n + 1, steps, peak)
    else Collatz(n div 2, steps, peak)
  end
end;
var s: Integer; p: LongInt; k: Integer;
begin
  for k := 1 to 12 do begin
    s := 0; p := 0;
    Collatz(k, s, p);
    WriteLn(k, ': ', s, ' ', p)
  end;
  s := 0; p := 0;
  Collatz(27, s, p);
  WriteLn(s, ' ', p)
end.
