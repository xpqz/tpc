program StringInsertTruncate;
var s: String[8]; t: String; i: Integer;
begin
  s := 'abcdef';
  Insert('123456', s, 3);
  WriteLn(s, ' ', Length(s));
  s := 'abcdef';
  Insert('XYZ', s, 7);
  WriteLn(s, ' ', Length(s));
  s := 'abcdef';
  Insert('XYZ', s, 9);
  WriteLn(s, ' ', Length(s));
  t := '';
  for i := 1 to 25 do Insert('0123456789', t, 1);
  WriteLn(Length(t));
  Insert('abc', t, 1);
  WriteLn(Length(t), ' ', Copy(t, 1, 5))
end.
