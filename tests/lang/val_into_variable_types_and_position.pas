program ValPos;
var s: String; i, e, n: Integer;
begin
  s := '12abc';
  Val(s, i, e);
  WriteLn(e);
  s := 'abc';
  Val(s, i, e);
  WriteLn(e);
  s := '42';
  Val(s, i, e);
  WriteLn(e, ' ', i);
  s := '9 9';
  Val(s, i, e);
  WriteLn(e);
  Val(Copy('x37y', 2, 2), i, e);
  WriteLn(i, ' ', e);
  Val('5' + '6', n, e);
  WriteLn(n + 1, ' ', e);
  n := 99;
  Val('zz', n, e);
  WriteLn(n, ' ', e)
end.
