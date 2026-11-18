program PtrResults;
type PStr = ^String;
function NewStr(s: String): PStr;
var p: PStr;
begin
  New(p);
  p^ := s;
  NewStr := p;
end;
function Longer(a, b: PStr): PStr;
begin
  if Length(a^) >= Length(b^) then Longer := a else Longer := b;
end;
var a, b, c: PStr;
begin
  a := NewStr('short');
  b := NewStr('a longer string');
  c := Longer(a, b);
  WriteLn(c^, ' ', c = b, ' ', Longer(a, a) = a);
  c^ := c^ + '!';
  WriteLn(b^);
  Dispose(a); Dispose(b)
end.
