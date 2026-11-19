program IncludeInsideARecordType;
type
  TBox = record
{$I FIELDS.INC}
  end;
var
  b: TBox;
begin
  b.w := 3;
  b.h := 4;
  WriteLn(b.w * b.h);
end.
