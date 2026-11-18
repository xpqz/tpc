program ValReal;
var r: Real; e: Integer;
procedure T(s: String);
begin
  r := 77;
  Val(s, r, e);
  WriteLn('[', s, '] ', r:0:4, ' ', e);
end;
begin
  T('1.5');
  T('0');
  T('-2.25');
  T('+0.125');
  T('100');
  T('1E3');
  T('1.5E2');
  T('2.5e-1');
  T('1E');
  T('1.');
  T('.5');
  T('abc');
  T('1.5x');
  T('');
  T('1.2.3');
  T('12345.5')
end.
