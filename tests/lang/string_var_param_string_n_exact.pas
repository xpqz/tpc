program StringVarParamN;
type S5 = String[5];
procedure Zap(var s: S5);
begin
  s := 'abcdefgh';
  s[1] := '*'
end;
var a: S5; b: S5;
begin
  Zap(a); Zap(b);
  WriteLn(a, ' ', b, ' ', Length(a))
end.
