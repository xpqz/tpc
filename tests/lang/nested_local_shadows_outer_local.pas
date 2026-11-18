program NestedLocalShadowsOuterLocal;
{ a nested routine's own local hides the enclosing routine's local of the same name }
procedure Outer;
var v: Integer;
  procedure Inner;
  var v: Integer;
  begin
    v := 99;
    WriteLn('inner ', v)
  end;
begin
  v := 1;
  Inner;
  WriteLn('outer ', v)
end;
begin
  Outer
end.
