program ide_keys_find_is_not_case_sensitive;
uses Crt, Buffer, View, Keys;

var
  Cmd: Integer;
  Log: string;

procedure Put(S: string);
var I: Integer;
begin
  for I := 1 to Length(S) do InsertChar(S[I]);
end;

procedure PutLine(S: string);
begin
  Put(S);
  InsertChar(LineBreak);
end;

procedure NumberedLines(Count: Integer);
var N: Integer; S: string;
begin
  for N := 1 to Count do
  begin
    Str(N, S);
    PutLine('line ' + S);
  end;
end;

procedure Fill;
begin
  PutLine('The cat sat on the mat.'); PutLine('The end.'); Put('Tail at');
end;

{ the loop of the editor: paint, read a command, note the commands that are not editing }
begin
  View.Reset;
  Fill;
  SetCursor(0);
  Log := '';
  repeat
    Paint;
    Cmd := ReadCommand;
    if (Cmd <> CmdNone) and (Cmd <> CmdQuit) then
    begin
      Log := Log + Chr(Ord('0') + Cmd);
      if MenuLetter <> #0 then Log := Log + MenuLetter;
      Log := Log + ' ';
      SetMessage('commands ' + Log);
      MenuLetter := #0;
    end;
  until Cmd = CmdQuit;

end.
