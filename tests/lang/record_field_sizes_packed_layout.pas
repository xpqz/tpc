program RecordLayout;
type
  A = record b: Byte; i: Integer; l: LongInt end;
  B = record c: Char; w: Word; s: ShortInt; f: Boolean end;
  C = record end;
  D = record s: String[5]; b: Byte end;
begin
  WriteLn(SizeOf(A), ' ', SizeOf(B), ' ', SizeOf(D))
end.
