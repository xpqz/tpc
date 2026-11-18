program ValInt;
var i, e: Integer;
procedure T(s: String);
begin
  Val(s, i, e);
  WriteLn('[', s, '] ', i, ' ', e);
end;
begin
  T('123');
  T('0');
  T('-45');
  T('+7');
  T('007');
  T('32767');
  T('-32768');
  T('12x');
  T('x12');
  T('');
  T(' 12');
  T('12 ');
  T('1 2');
  T('-');
  T('$FF');
  T('$7FFF');
  T('$8000');
  T('1.5');
  T('--1');
end.
