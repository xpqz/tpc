program ForCharAndBooleanControl;
var
  c: Char;
  b: Boolean;
begin
  for c := 'a' to 'f' do Write(c);
  WriteLn;
  for c := 'f' downto 'a' do Write(c);
  WriteLn;
  for c := '0' to '9' do Write(c);
  WriteLn;
  for b := False to True do Write(b, ' ');
  WriteLn;
  for b := True downto False do Write(b, ' ');
  WriteLn
end.
