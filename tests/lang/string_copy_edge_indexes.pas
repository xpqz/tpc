program StringCopyEdge;
var s: String; i, j: Integer;
begin
  s := 'abcdef';
  for i := 0 to 8 do begin
    for j := 0 to 7 do Write(Length(Copy(s, i, j)));
    WriteLn
  end;
  s := '';
  WriteLn(Length(Copy(s, 1, 1)), Length(Copy(s, 0, 0)))
end.
