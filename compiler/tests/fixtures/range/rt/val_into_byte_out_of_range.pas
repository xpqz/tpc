program ValByte;
{$R+}
var b: Byte; c: Integer;
begin
  Val('255', b, c);
  WriteLn(b, ' ', c);
  Val('300', b, c);
  WriteLn('not reached')
end.
