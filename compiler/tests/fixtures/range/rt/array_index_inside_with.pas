program WithIndex;
{$R+}
type Rec = record a: array[1..3] of Integer end;
var r: Rec; i: Integer;
begin
  i := 3;
  with r do a[i] := 7;
  WriteLn(r.a[3]);
  i := 4;
  with r do a[i] := 8;
  WriteLn('not reached')
end.
