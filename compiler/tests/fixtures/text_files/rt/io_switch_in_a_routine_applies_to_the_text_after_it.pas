program IoSwitchInARoutineAppliesToTheTextAfterIt;
{ the switch is positional in the source: after the routine the main block is still unchecked }
var
  f: Text;
  e: Integer;

procedure Quiet;
begin
  {$I-}
end;

begin
  Assign(f, 'NONE.TXT');
  Reset(f);
  e := IOResult;
  WriteLn('code ', e);
  {$I+}
  Reset(f);
  WriteLn('not reached');
end.
