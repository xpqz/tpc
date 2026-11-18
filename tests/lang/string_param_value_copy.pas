program StringParamValue;
procedure Mut(s: String);
begin
  s := s + '!!';
  s[1] := 'X';
  WriteLn(s)
end;
procedure MutVar(var s: String);
begin
  s := s + '!!';
  s[1] := 'X'
end;
type S3 = String[3];
procedure Short(s: S3);
begin
  WriteLn(s, Length(s))
end;
var a: String; b: S3;
begin
  a := 'hello';
  Mut(a);
  WriteLn(a);
  MutVar(a);
  WriteLn(a);
  Short('abcdef');
  Short(a);
  b := 'xyz';
  Mut(b);
  WriteLn(b)
end.
