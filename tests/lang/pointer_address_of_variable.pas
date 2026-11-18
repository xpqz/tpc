program PtrAddr;
type PI = ^Integer;
var i, j: Integer; p: PI; a: array[1..5] of Integer; k: Integer;
begin
  i := 5;
  p := @i;
  p^ := 7;
  WriteLn(i);
  i := 11;
  WriteLn(p^);
  j := p^ + 1;
  WriteLn(j);
  WriteLn(p = @i, ' ', p = @j);
  for k := 1 to 5 do a[k] := k * 10;
  p := @a[3];
  WriteLn(p^);
  p^ := -1;
  WriteLn(a[3]);
  p := @a[1];
  p^ := p^ + 1;
  WriteLn(a[1])
end.
