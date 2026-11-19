program UserRtError;
var z: Integer;

procedure RtError(code: LongInt);
begin
  WriteLn('user routine called')
end;

begin
  z := 0;
  Write('before ');
  WriteLn(10 div z)
end.
