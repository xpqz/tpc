program ide_menu_bar_is_drawn_over_the_placeholder;
uses Crt, Buffer, View, Keys, Menu;

var
  Cmd: Integer;
  Log: string;

procedure Put(S: string);
var I: Integer;
begin
  for I := 1 to Length(S) do InsertChar(S[I]);
end;

procedure NumberedLines(Count: Integer);
var N: Integer; S: string;
begin
  for N := 1 to Count do
  begin
    Str(N, S);
    Put('row ' + S + ' abcdefghijklmnopqrstuvwxyz ABCDEFGHIJKLMNOPQRSTUVWXYZ 0123456789 ....');
    InsertChar(LineBreak);
  end;
end;

{ the loop of the editor: paint, draw the bar, read a command, run the menu for CmdMenu }
begin
  View.Reset;
  NumberedLines(5);
  SetCursor(0);
  
  Log := '';
  repeat
    Paint;
    DrawBar;
    Cmd := ReadCommand;
    if Cmd = CmdMenu then
      Cmd := Menu.Run(MenuLetter);
    if (Cmd <> CmdNone) and (Cmd <> CmdQuit) then
      Log := Log + Chr(Ord('0') + Cmd) + ' ';
    if Log <> '' then
      SetMessage('commands ' + Log);
  until Cmd = CmdQuit;
  
end.
