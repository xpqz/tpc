program StringNested;
function Outer(s: String): String;
  function Inner(t: String): String;
  begin Inner := t + s end;
begin Outer := Inner('<') + Inner('>') end;
procedure Grow(var s: String; n: Integer);
  function Mk: String; begin Mk := s + '.' end;
begin
  while n > 0 do begin s := Mk; n := n - 1 end
end;
var s: String;
begin
  WriteLn(Outer('ab'));
  s := 'x'; Grow(s, 4);
  WriteLn(s)
end.
