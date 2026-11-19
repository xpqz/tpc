{ The editor loop the Files tests share (test code, not part of the IDE). Alt-F stands for
  File New because the menu is not under test. Each command key does what the IDE's main loop
  will do with it: F2 saves, F3 opens, Alt-F creates a new file, Alt-X quits unless the user
  cancels the question about unsaved changes.

  Report puts a description of the session above the text: the current name, whether the text
  was modified, how long it is, and the contents of the named files as `|line|` rows, so the
  files a test wrote are checked by reading them back. A text longer than MaxShownText is
  dropped from the screen (painting 64 KB is slow). The screen is never cleared. }
unit FHarness;

interface

{ Starts a session whose text is Prefill (`~` is a line break), unmodified and unnamed. }
procedure Start(Prefill: string);
procedure EditLoop;
{ Names is a blank-separated list of files to show; at most 14 lines of each. }
procedure Report(Names: string);

implementation

uses Crt, Buffer, View, Keys, Files;

const
  MaxShownText = 600;
  MaxShownLines = 14;

procedure PutText(S: string);
var
  I: Integer;
begin
  for I := 1 to Length(S) do
    InsertChar(S[I]);
end;

procedure Start(Prefill: string);
var
  I: Integer;
begin
  View.Reset;
  ResetKeys;
  Clear;
  for I := 1 to Length(Prefill) do
    if Prefill[I] = '~' then InsertChar(LineBreak) else InsertChar(Prefill[I]);
  SetCursor(0);
  ResetFiles;
end;

procedure EditLoop;
var
  Cmd: Integer;
  Done: Boolean;
begin
  Done := False;
  repeat
    Paint;
    Cmd := ReadCommand;
    if Cmd = CmdSave then
    begin
      if SaveFile then;
    end
    else if Cmd = CmdOpen then
      OpenFile
    else if (Cmd = CmdMenu) and (MenuLetter = 'F') then
      NewFile
    else if Cmd = CmdQuit then
      Done := SaveIfModified;
    MenuLetter := #0;
  until Done;
end;

{ Types `== NAME` and the first lines of the file as `|line|`, or `== NAME missing`. }
procedure ShowFile(Name: string);
var
  F: Text;
  Line: string;
  Shown: Integer;
begin
  PutText('== ' + Name);
  {$I-}
  Assign(F, Name);
  System.Reset(F);
  {$I+}
  if IOResult <> 0 then
    PutText(' missing')
  else
  begin
    Shown := 0;
    while not Eof(F) and (Shown < MaxShownLines) do
    begin
      ReadLn(F, Line);
      InsertChar(LineBreak);
      PutText('|' + Line + '|');
      Inc(Shown);
    end;
    if not Eof(F) then
    begin
      InsertChar(LineBreak);
      PutText('...');
    end;
    Close(F);
  end;
  InsertChar(LineBreak);
end;

procedure Report(Names: string);
var
  Head, Name, Count: string;
  Blank: Integer;
begin
  Head := 'name=' + CurrentName;
  if IsModified then Head := Head + ' modified' else Head := Head + ' unmodified';
  Str(TextLength, Count);
  Head := Head + ' chars=' + Count;
  if TextLength > MaxShownText then
    Clear;
  SetCursor(0);
  PutText(Head);
  InsertChar(LineBreak);
  while Names <> '' do
  begin
    Blank := Pos(' ', Names);
    if Blank = 0 then Blank := Length(Names) + 1;
    Name := Copy(Names, 1, Blank - 1);
    Delete(Names, 1, Blank);
    ShowFile(Name);
  end;
  PutText('-- text --');
  InsertChar(LineBreak);
  SetCursor(0);
  Paint;
end;

end.
