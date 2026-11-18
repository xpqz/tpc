program StrValRound;
var s: String; i, e, k: Integer; l, m: LongInt; bad: Integer;
begin
  bad := 0;
  for k := -50 to 50 do begin
    i := k * 600;
    Str(i, s);
    Val(s, m, e);
    if (m <> i) or (e <> 0) then bad := bad + 1;
  end;
  l := 1;
  for k := 1 to 9 do begin
    l := l * 10 + k;
    Str(l, s);
    Val(s, m, e);
    if (m <> l) or (e <> 0) then bad := bad + 1;
    Str(-l, s);
    Val(s, m, e);
    if (m <> -l) or (e <> 0) then bad := bad + 1;
  end;
  WriteLn(bad)
end.
