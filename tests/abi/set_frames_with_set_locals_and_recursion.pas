program SetFrames;
type TS = set of 0..31;
function Depth(n: Integer; s: TS): Integer;
var local: TS; k: Integer;
begin
  local := s + [n];
  if n = 0 then begin
    k := 0;
    for n := 0 to 31 do if n in local then k := k + 1;
    Depth := k;
  end else Depth := Depth(n - 1, local);
end;
var s: TS;
begin
  s := [];
  WriteLn(Depth(20, s));
  WriteLn(Depth(31, [0, 31]));
  s := [5, 6];
  WriteLn(Depth(10, s), ' ', 5 in s, ' ', 10 in s)
end.
