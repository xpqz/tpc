program ProcVarLocalVariable;
{$F+}
{ a procedural variable declared as a local and as a captured local }
type TF = function(x: Integer): Integer;
function Dbl(x: Integer): Integer; begin Dbl := x * 2 end;
function Sq(x: Integer): Integer; begin Sq := x * x end;
procedure Outer(pick: Boolean);
var f: TF;
  procedure Show(x: Integer);
  begin
    WriteLn(f(x))
  end;
begin
  if pick then f := Dbl else f := Sq;
  Show(6);
  f := Dbl;
  Show(6)
end;
procedure Plain(pick: Boolean);
var f: TF;
begin
  if pick then f := Dbl else f := Sq;
  WriteLn(f(9))
end;
begin
  Outer(True);
  Outer(False);
  Plain(True);
  Plain(False)
end.
