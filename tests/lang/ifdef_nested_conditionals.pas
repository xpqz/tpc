program IfdefNestedConditionals;
{$DEFINE A}
{$DEFINE B}
begin
{$IFDEF A}
  WriteLn('a');
  {$IFDEF B}
  WriteLn('a and b');
  {$ELSE}
  WriteLn('a not b');
  {$ENDIF}
  {$IFDEF C}
  WriteLn('a and c');
  {$ELSE}
  WriteLn('a not c');
    {$IFNDEF D}
  WriteLn('a not c not d');
    {$ENDIF}
  {$ENDIF}
{$ELSE}
  WriteLn('not a');
  {$IFDEF B}
  WriteLn('never');
  {$ENDIF}
{$ENDIF}
  WriteLn('end');
end.
