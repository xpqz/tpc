program UnitPrivateNamesDoNotClashWithProgram;
{ names declared only in the implementation part are invisible outside the unit }
uses Hidden;
var
  Secret: Integer;
procedure Helper;
begin
  WriteLn('program helper');
end;
begin
  Secret := 5;
  Helper;
  WriteLn(Reveal);
  WriteLn(Secret);
end.
