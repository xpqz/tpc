program NestedVarParamForwardedFromDepthThree;
{ a var parameter of the outermost routine is read, written and passed on as a var argument
  by a routine three levels down; the caller's variable changes }
procedure Bump(var x: Integer; by: Integer);
begin
  x := x + by
end;
procedure Outer(var total: Integer; step: Integer);
  procedure Mid;
    procedure Inner;
    begin
      Bump(total, step);
      total := total * 2;
      Bump(total, 1)
    end;
  begin
    Inner;
    Inner
  end;
begin
  Mid;
  Bump(total, 1000)
end;
var t: Integer;
begin
  t := 1;
  Outer(t, 3);
  WriteLn(t);
  Outer(t, -50);
  WriteLn(t)
end.
