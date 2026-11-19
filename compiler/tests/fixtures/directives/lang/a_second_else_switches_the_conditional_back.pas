program SecondElse;
begin
{$IFDEF X}
  WriteLn('then');
{$ELSE}
  WriteLn('else 1');
{$ELSE}
  WriteLn('else 2');
{$ENDIF}
  WriteLn('after');
end.
