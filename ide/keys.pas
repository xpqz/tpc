{ Keys: reads one key and turns it into an editing action on the Buffer.

  ReadCommand blocks for a key (ReadKey returns an extended key as #0 followed by its scan
  code), edits the Buffer and tells the caller about the keys that are not editing: F2 save,
  F3 open, F9 compile, Ctrl-F9 run, F10 and Alt-F/E/R/C/O the menu, Alt-X quit. Painting is
  the caller's job, except while the find prompt is open.

  Block commands (^K B begin, ^K K end, ^K C copy, ^K V move) work on one block marked by two
  buffer positions. Any other edit hides the block, so the marks never point into text that
  has moved. A browser cannot send ^K and ^Q, so Alt-K and Alt-Q stand for them (decision 41). }
unit Keys;

interface

const
  CmdNone = 0;          { the key was an edit, a cursor move or was ignored }
  CmdSave = 1;
  CmdOpen = 2;
  CmdCompile = 3;
  CmdRun = 4;
  CmdMenu = 5;          { MenuLetter is the pull-down to open, or #0 for F10 }
  CmdQuit = 6;
  CmdNew = 7;           { File, New; only the menu gives it }

var
  MenuLetter: Char;

function ReadCommand: Integer;
procedure ResetKeys;

implementation

uses Crt, Buffer, View;

const
  PageLines = 23;       { the height of the edit area }
  NoMark = -1;
  BackspaceKey = #8;
  TabKey = #9;
  CtrlL = #12;
  EnterKey = #13;
  CtrlY = #25;
  EscKey = #27;

var
  Overwrite: Boolean;
  BlockStart, BlockEnd: LongInt;   { buffer positions; the block is [BlockStart, BlockEnd) }
  FindText: string;

{ Forgets the block, the find text and Overwrite mode. }
procedure ResetKeys;
begin
  Overwrite := False;
  MenuLetter := #0;
  BlockStart := NoMark;
  BlockEnd := NoMark;
  FindText := '';
  SetInsertMode(True);
end;

procedure HideBlock;
begin
  BlockStart := NoMark;
  BlockEnd := NoMark;
end;

function BlockIsMarked: Boolean;
begin
  BlockIsMarked := (BlockStart <> NoMark) and (BlockEnd <> NoMark) and (BlockStart < BlockEnd);
end;

{ ---- Typing ---- }

{ In Overwrite mode a typed character replaces the one under the cursor, except at the end
  of a line, where it is inserted. }
procedure TypeChar(Ch: Char);
begin
  if Overwrite and (Cursor < TextLength) and (CharAt(Cursor) <> LineBreak) then
    DeleteAfter;
  InsertChar(Ch);
  HideBlock;
end;

{ Enter: breaks the line and gives the new line the blanks that start the line being left, as
  many as lie to the left of the cursor (auto-indent). Text before the cursor does not move
  when the break is inserted, so Scan stays valid. }
procedure BreakLine;
var
  Scan, Left: LongInt;
begin
  if not IsFull then
  begin
    Left := Cursor;
    Scan := LineStartPos(Left);
    InsertChar(LineBreak);
    while (Scan < Left) and (CharAt(Scan) in [' ', TabKey]) do
    begin
      InsertChar(CharAt(Scan));
      Inc(Scan);
    end;
    HideBlock;
  end;
end;

{ ^Y: removes the whole line, its line break included, and leaves the cursor at its start. }
procedure DeleteLine;
var
  Start, Finish: LongInt;
begin
  Start := LineStartPos(Cursor);
  Finish := LineEndPos(Cursor);
  if Finish < TextLength then
    Inc(Finish);
  SetCursor(Start);
  while Finish > Start do
  begin
    DeleteAfter;
    Dec(Finish);
  end;
  HideBlock;
end;

procedure MoveUpLines(Count: Integer);
begin
  while (Count > 0) and (LineOf(Cursor) > 1) do
  begin
    MoveUp;
    Dec(Count);
  end;
end;

procedure MoveDownLines(Count: Integer);
begin
  while (Count > 0) and (LineOf(Cursor) < LineCount) do
  begin
    MoveDown;
    Dec(Count);
  end;
end;

procedure ToggleOverwrite;
begin
  Overwrite := not Overwrite;
  SetInsertMode(not Overwrite);
end;

{ ---- Block ---- }

{ Inserts a copy of the block at the cursor, marks the copy as the block and puts the cursor
  at its start. OriginalStart is where the original block starts afterwards. When the
  cursor is at or before the block, each inserted character moves the original one cell to
  the right, so the cell to copy next moves with it. }
procedure InsertBlockCopy(var OriginalStart: LongInt);
var
  Size, Target, Done, Shift: LongInt;
begin
  Size := BlockEnd - BlockStart;
  Target := Cursor;
  OriginalStart := BlockStart;
  if Target <= BlockStart then
    OriginalStart := BlockStart + Size;
  for Done := 0 to Size - 1 do
  begin
    Shift := 0;
    if Target <= BlockStart then
      Shift := Done;
    InsertChar(CharAt(BlockStart + Shift + Done));
  end;
  BlockStart := Target;
  BlockEnd := Target + Size;
  SetCursor(Target);
end;

{ Removes Size characters at Start. }
procedure DeleteRange(Start, Size: LongInt);
begin
  SetCursor(Start);
  while Size > 0 do
  begin
    DeleteAfter;
    Dec(Size);
  end;
end;

{ ^K C copies the block to the cursor, ^K V moves it. The cursor must be outside the block
  (at its ends is allowed) and the copy must fit in the buffer. }
procedure CopyOrMoveBlock(Move: Boolean);
var
  OriginalStart, Size: LongInt;
begin
  Size := BlockEnd - BlockStart;
  if not BlockIsMarked then
    SetMessage('No block marked')
  else if (Cursor > BlockStart) and (Cursor < BlockEnd) then
    SetMessage('Cursor is inside the block')
  else if TextLength + Size > MaxText then
    SetMessage('Not enough room')
  else
  begin
    InsertBlockCopy(OriginalStart);
    if Move then
    begin
      DeleteRange(OriginalStart, Size);
      if OriginalStart < BlockStart then
      begin
        Dec(BlockStart, Size);
        Dec(BlockEnd, Size);
      end;
      SetCursor(BlockStart);
    end;
  end;
end;

{ ---- Find ---- }

{ True when the find text is in the buffer at Pos; capitals and small letters are equal. }
function FindTextAt(Pos: LongInt): Boolean;
var
  Index: Integer;
  Same: Boolean;
begin
  Same := True;
  for Index := 1 to Length(FindText) do
    if UpCase(CharAt(Pos + Index - 1)) <> UpCase(FindText[Index]) then
      Same := False;
  FindTextAt := Same;
end;

{ Searches forward from the cursor and puts the cursor after the match. }
procedure FindNext;
var
  Pos: LongInt;
  Found: Boolean;
begin
  Found := False;
  Pos := Cursor;
  while (not Found) and (Pos + Length(FindText) <= TextLength) do
  begin
    if FindTextAt(Pos) then
      Found := True
    else
      Inc(Pos);
  end;
  if Found then
    SetCursor(Pos + Length(FindText))
  else
    SetMessage('Search string not found');
end;

{ ^Q F: asks for the text, then finds it. An empty answer searches for the last text again. }
procedure FindCommand;
var
  Answer: string;
begin
  if ReadPrompt('Find: ', Answer) then
  begin
    if Answer <> '' then
      FindText := Answer;
    if FindText <> '' then
      FindNext;
  end;
end;

{ ---- Dispatch ---- }

{ The letter of a key typed after ^K or ^Q: the control code ^B counts as B. }
function PrefixedLetter(Key: Char): Char;
begin
  if (Key >= #1) and (Key <= #26) then
    Key := Chr(Ord(Key) + 64);
  PrefixedLetter := UpCase(Key);
end;

{ The key after ^K (or Alt-K). Extended keys are read to the end and ignored. }
procedure BlockCommand;
var
  Key: Char;
begin
  Key := ReadKey;
  if Key = #0 then
    Key := ReadKey
  else
    case PrefixedLetter(Key) of
      'B': BlockStart := Cursor;
      'K': BlockEnd := Cursor;
      'C': CopyOrMoveBlock(False);
      'V': CopyOrMoveBlock(True);
    end;
end;

procedure QuickCommand;
var
  Key: Char;
begin
  Key := ReadKey;
  if Key = #0 then
    Key := ReadKey
  else if PrefixedLetter(Key) = 'F' then
    FindCommand;
end;

{ Handles the key after #0, returns the command for the keys that are not editing. }
function ExtendedCommand(Scan: Char): Integer;
begin
  ExtendedCommand := CmdNone;
  case Ord(Scan) of
    72: MoveUp;
    80: MoveDown;
    75: MoveLeft;
    77: MoveRight;
    71: MoveToLineStart;
    79: MoveToLineEnd;
    73: MoveUpLines(PageLines);
    81: MoveDownLines(PageLines);
    82: ToggleOverwrite;
    83:
      begin
        DeleteAfter;
        HideBlock;
      end;
    37: BlockCommand;
    16: QuickCommand;
    60: ExtendedCommand := CmdSave;
    61: ExtendedCommand := CmdOpen;
    67: ExtendedCommand := CmdCompile;
    102: ExtendedCommand := CmdRun;
    68:
      begin
        MenuLetter := #0;
        ExtendedCommand := CmdMenu;
      end;
    45: ExtendedCommand := CmdQuit;
    33, 18, 19, 46, 24:
      begin
        case Ord(Scan) of
          33: MenuLetter := 'F';
          18: MenuLetter := 'E';
          19: MenuLetter := 'R';
          46: MenuLetter := 'C';
          24: MenuLetter := 'O';
        end;
        ExtendedCommand := CmdMenu;
      end;
  end;
end;

{ Reads one key and does what it says. The status message is cleared first, so a message
  set by the command stays until the next key. }
function ReadCommand: Integer;
var
  Key: Char;
begin
  Key := ReadKey;
  SetMessage('');
  ReadCommand := CmdNone;
  case Key of
    #0: ReadCommand := ExtendedCommand(ReadKey);
    #11: BlockCommand;
    #17: QuickCommand;
    CtrlL: if FindText <> '' then FindNext;
    CtrlY: DeleteLine;
    EnterKey: BreakLine;
    BackspaceKey:
      begin
        DeleteBefore;
        HideBlock;
      end;
    TabKey: TypeChar(TabKey);
    ' '..#255: TypeChar(Key);
  end;
end;

begin
  ResetKeys;
end.
