program P;
type Rec = record case Boolean of True: (a: Integer); False: (b: Byte) end;
var rv: Rec;
begin
  rv.c := 1
end.
