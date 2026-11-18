program EnumInRecord;
type
  Kind = (K0, K1, K2, K3);
  Rec = packed record a: Kind; b: Byte; c: Kind end;
var r: Rec;
begin
  WriteLn(SizeOf(Kind), ' ', SizeOf(Rec));
  r.a := K3; r.b := 200; r.c := K1;
  WriteLn(Ord(r.a), ' ', r.b, ' ', Ord(r.c))
end.
