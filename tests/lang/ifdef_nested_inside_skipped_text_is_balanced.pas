program IfdefNestedInsideSkippedTextIsBalanced;
{ conditionals inside skipped text are counted, so their ELSE and ENDIF do not end the outer one }
begin
{$IFDEF OUTER}
  {$IFDEF INNER}
  WriteLn('inner');
  {$ELSE}
  WriteLn('not inner');
  {$ENDIF}
  WriteLn('outer');
{$ELSE}
  WriteLn('no outer');
{$ENDIF}
  WriteLn('after');
end.
