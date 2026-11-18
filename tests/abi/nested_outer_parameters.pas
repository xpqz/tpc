program NestedOuterParameters;
{ nested routines read and write the value and var parameters of the enclosing routine }
procedure Outer(p: Integer; var q: Integer; r: Byte);
  procedure Inner;
  begin
    p := p + 1;
    q := q * 2;
    r := r + 200
  end;
begin
  Inner;
  Inner;
  WriteLn(p, ' ', q, ' ', r)
end;
var g: Integer;
begin
  g := 7;
  Outer(10, g, 100);
  WriteLn(g)
end.
