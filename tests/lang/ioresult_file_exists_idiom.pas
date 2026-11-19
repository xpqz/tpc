program IoresultFileExistsIdiom;
var
  f: Text;
function Exists(name: string): Boolean;
var
  f: Text;
begin
  {$I-}
  Assign(f, name);
  Reset(f);
  if IOResult = 0 then
  begin
    Close(f);
    Exists := IOResult = 0;
  end
  else
    Exists := False;
  {$I+}
end;
begin
  WriteLn(Exists('PRESENT.TXT'));
  WriteLn(Exists('ABSENT.TXT'));
  WriteLn(Exists('present.txt'));
  Assign(f, 'ABSENT.TXT');
  Rewrite(f);
  Close(f);
  WriteLn(Exists('ABSENT.TXT'));
end.
