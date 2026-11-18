program WriteMultipleArguments;
var n: Integer;
begin
  n := 7;
  Write('n=', n, '; ');
  Write(n, n, n);
  WriteLn;
  WriteLn('a', 1, 'b', 2, True, 'c', n);
  WriteLn(1, 2, 3)
end.
