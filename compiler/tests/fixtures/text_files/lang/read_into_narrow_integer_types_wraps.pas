program ReadIntoNarrowIntegerTypesWraps;
{ Read stores the number modulo the size of the target; there is no range check }
var
  f: Text;
  b: Byte;
  sh: ShortInt;
  w: Word;
  i: Integer;
  code: Integer;
begin
  Assign(f, 'WRAP.TXT');
  Reset(f);
  Read(f, b, sh, w, i);
  WriteLn(b, ' ', sh, ' ', w, ' ', i);
  Read(f, w, i, b);
  WriteLn(w, ' ', i, ' ', b);
  Close(f);
end.
