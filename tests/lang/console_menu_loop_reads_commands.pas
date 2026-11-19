program ConsoleMenuLoopReadsCommands;
var
  cmd: string;
  total: Integer;
  n: Integer;
begin
  total := 0;
  repeat
    Write('> ');
    ReadLn(cmd);
    if cmd = 'add' then
    begin
      Write('value? ');
      ReadLn(n);
      total := total + n;
    end
    else if cmd = 'show' then
      WriteLn(total)
    else if cmd <> 'quit' then
      WriteLn('unknown: ', cmd);
  until cmd = 'quit';
  WriteLn('bye');
end.
