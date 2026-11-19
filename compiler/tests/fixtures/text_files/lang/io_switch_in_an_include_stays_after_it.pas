program IoSwitchInAnIncludeStaysAfterIt;
{ an I- switch read from an include file is still in force after the include }
{$I NOCHECK.INC}
var
  f: Text;
  code: Integer;
begin
  Assign(f, 'MISSING.TXT');
  Reset(f);
  code := IOResult;
  WriteLn('code ', code);
end.
