program StringResultArg;
function Mk(c: Char): String; begin Mk := c + c end;
function Len(s: String): Integer; begin Len := Length(s) end;
function Join(a, b, c: String): String; begin Join := a + '/' + b + '/' + c end;
procedure Show(s: String); begin WriteLn('<', s, '>') end;
procedure ShowVar(var s: String); begin s := s + '!'; WriteLn('<', s, '>') end;
var v: String;
begin
  Show(Mk('a'));
  WriteLn(Len(Mk('q')), Len(Join(Mk('a'), Mk('b'), Mk('c'))));
  Show(Join(Mk('x'), Join('1', '2', '3'), Mk('y')));
  v := Mk('v'); ShowVar(v); ShowVar(v)
end.
