program FieldTarget;
{$R+}
type Rec = record n: Integer; b: 1..3 end;
var r: Rec; i: Integer;
begin
  i := 3;
  r.b := i;
  WriteLn(r.b);
  i := 4;
  with r do b := i;
  WriteLn('not reached')
end.
