program TextfileReadMixedFieldsOfARecordLine;
var
  f: Text;
  id: Integer;
  c: Char;
  name: string[5];
  price: Real;
begin
  Assign(f, 'REC.TXT');
  Reset(f);
  while not Eof(f) do
  begin
    ReadLn(f, id, c, name, price);
    WriteLn(id, '|', Ord(c), '|', name, '|', price:0:2);
  end;
  Close(f);
end.
