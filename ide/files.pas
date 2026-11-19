{ Files: the current file of the editor and the commands New, Open and Save.

  The editor has one text, the Buffer, and one file name. A text that was never saved is
  called NONAME.PAS and the first Save asks for a name. Typed names are upper-cased (DOS names
  are not case sensitive) and get the extension .PAS when they have none.

  "Modified" is the Buffer's own flag: set by every edit, cleared when the text is cleared,
  loaded or saved. Every question is asked on the status line with View.ReadPrompt, and so are
  the errors. Reading and writing a file is the job of Buffer.Load and Buffer.Save; a saved file
  lives in the host's file system, which keeps it for the next session. }
unit Files;

interface

const
  DefaultName = 'NONAME.PAS';

function CurrentName: string;
function IsModified: Boolean;

{ Treats the text in the Buffer as unmodified and unnamed (the start of a session). }
procedure ResetFiles;

{ Clears the Buffer and forgets the name; asks first if the text is modified. }
procedure NewFile;

{ Asks for a name and loads that file; asks first if the text is modified. }
procedure OpenFile;

{ Writes the text to the current name, asking for a name if there is none. True on success. }
function SaveFile: Boolean;

{ True when the caller may go on and replace the text: it is unmodified, or the user chose to
  save it (and the save worked) or to discard it. False when the user cancelled. }
function SaveIfModified: Boolean;

implementation

uses Crt, Buffer, View;

const
  EscKey = #27;
  DosFileNotFound = 2;
  DosPathNotFound = 3;

var
  Name: string;
  HasName: Boolean;

function CurrentName: string;
begin
  CurrentName := Name;
end;

function IsModified: Boolean;
begin
  IsModified := Modified;
end;

{ Records that the text in the Buffer is what the file called NewName holds. }
procedure Adopt(NewName: string; Named: Boolean);
begin
  Name := NewName;
  HasName := Named;
  SetModified(False);
end;

procedure ResetFiles;
begin
  Adopt(DefaultName, False);
end;

{ Upper-cases the typed name and adds .PAS when it has no extension. }
function FullName(Typed: string): string;
var
  I: Integer;
begin
  for I := 1 to Length(Typed) do
    Typed[I] := UpCase(Typed[I]);
  if Pos('.', Typed) = 0 then
    Typed := Typed + '.PAS';
  FullName := Typed;
end;

{ Asks for a file name; True when a non-empty name was accepted. }
function AskName(Prompt: string; var Typed: string): Boolean;
begin
  AskName := ReadPrompt(Prompt, Typed) and (Typed <> '');
end;

function SaveFile: Boolean;
var
  Typed, Target: string;
begin
  SaveFile := False;
  Target := Name;
  if not HasName then
  begin
    if not AskName('Save file as: ', Typed) then
      Exit;
    Target := FullName(Typed);
  end;
  if Save(Target) <> 0 then
    SetMessage('Cannot write file')
  else
  begin
    Adopt(Target, True);
    SetMessage('Saved ' + Target);
    SaveFile := True;
  end;
end;

function SaveIfModified: Boolean;
var
  Key: Char;
  Decided: Boolean;
begin
  if not IsModified then
  begin
    SaveIfModified := True;
    Exit;
  end;
  SaveIfModified := False;
  Decided := False;
  while not Decided do
  begin
    SetMessage('Save changes to ' + Name + '? (Y/N)');
    Paint;
    Key := ReadKey;
    if Key = #0 then
      Key := ReadKey
    else if (UpCase(Key) = 'Y') or (UpCase(Key) = 'N') or (Key = EscKey) then
    begin
      Decided := True;
      SetMessage('');
      if UpCase(Key) = 'Y' then
        SaveIfModified := SaveFile
      else
        SaveIfModified := Key <> EscKey;
    end;
  end;
end;

procedure NewFile;
begin
  if SaveIfModified then
  begin
    Clear;
    ResetFiles;
  end;
end;

{ Loads Target into the Buffer. A missing file leaves everything as it was. A file that is
  too large leaves the Buffer empty, so the old name is dropped: saving the empty text under
  it would destroy the old file. }
procedure LoadFile(Target: string);
var
  Code: Integer;
begin
  Code := Load(Target);
  if Code = 0 then
    Adopt(Target, True)
  else if Code = LoadTooLarge then
  begin
    ResetFiles;
    SetMessage('File too large');
  end
  else if (Code = DosFileNotFound) or (Code = DosPathNotFound) then
    SetMessage('File not found')
  else
    SetMessage('Cannot read file');
end;

procedure OpenFile;
var
  Typed: string;
begin
  if AskName('Open file: ', Typed) then
    if SaveIfModified then
      LoadFile(FullName(Typed));
end;

begin
  ResetFiles;
end.
