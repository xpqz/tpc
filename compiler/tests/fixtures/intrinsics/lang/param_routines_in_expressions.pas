program ParamUse;
var i: Integer; l: LongInt; s: String; n: Word;
function Count: Integer;
begin
  Count := ParamCount
end;
begin
  WriteLn(ParamCount + 1);
  WriteLn(Count);
  WriteLn(ParamCount = 0, ' ', ParamCount < 1, ' ', Odd(ParamCount));
  Str(ParamCount, s);
  WriteLn('[', s, ']');
  l := 5;
  WriteLn('[', ParamStr(l), ']');
  n := 3;
  WriteLn('[', ParamStr(n), ']');
  s := ParamStr(1) + 'x' + ParamStr(2);
  WriteLn(s, ' ', Length(s));
  for i := 1 to ParamCount do WriteLn('arg ', ParamStr(i));
  i := 0;
  while i < ParamCount do Inc(i);
  WriteLn(i);
  Val(ParamStr(1), i, n);
  WriteLn(i, ' ', n);
  Inc(i, ParamCount);
  WriteLn(i);
  if ParamStr(1) = '' then WriteLn('empty')
end.
