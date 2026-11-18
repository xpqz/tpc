program RangeLoop;
{$R+}
var a: array[0..9] of Integer; i, sum: Integer;
begin
  for i := 0 to 9 do a[i] := i;
  sum := 0;
  for i := 0 to 10 do begin
    sum := sum + a[i];
    Write(sum, ' ');
  end;
  WriteLn;
  WriteLn('not reached')
end.
