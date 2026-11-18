program WithVarParam;
type R = record n: Integer; c: Char end;
procedure Fill(var r: R; k: Integer);
begin
  with r do begin n := k; c := Chr(64 + k) end
end;
function Total(var r: R): Integer;
begin
  with r do Total := n * 2
end;
var a: R;
begin
  Fill(a, 3);
  WriteLn(a.n, a.c, ' ', Total(a));
  with a do begin Fill(a, 5); WriteLn(n, c) end
end.
