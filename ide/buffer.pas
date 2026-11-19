{ Buffer: the text being edited, kept as a gap buffer.

  A gap buffer is one block of memory holding the text with a hole (the gap) in it:

      text before the gap | ........gap........ | text after the gap

  Typing at the gap costs nothing: the new character goes into the first free cell and the
  gap shrinks by one. Moving the gap to another place costs a copy of the characters it
  passes over, so the gap is moved only when the next edit needs it, not on every cursor
  move. Positions are 0-based offsets into the text as if the gap were not there (position 0
  is before the first character, TextLength after the last). Lines and columns are 1-based.
  Lines are separated by LineBreak; the last line has no break after it.

  Every position, size and index is a LongInt: a Word wraps silently at 65536 (decision 44). }
unit Buffer;

interface

const
  MaxText = 65521;      { the largest block TP 5.0 can allocate }
  LineBreak = #10;
  LoadTooLarge = -1;    { Load: the file does not fit in MaxText characters }

procedure Clear;
function TextLength: LongInt;
function Cursor: LongInt;
function CharAt(Pos: LongInt): Char;
function TextPointer: Pointer;

procedure SetCursor(Pos: LongInt);
procedure MoveLeft;
procedure MoveRight;
procedure MoveUp;
procedure MoveDown;
procedure MoveToLineStart;
procedure MoveToLineEnd;

function IsFull: Boolean;
procedure InsertChar(Ch: Char);
procedure DeleteBefore;
procedure DeleteAfter;

function LineCount: LongInt;
function LineOf(Pos: LongInt): LongInt;
function ColumnOf(Pos: LongInt): LongInt;
function LineStartPos(Pos: LongInt): LongInt;
function LineEndPos(Pos: LongInt): LongInt;
function PosOf(Line, Column: LongInt): LongInt;

function Load(Name: string): Integer;
function Save(Name: string): Integer;

implementation

type
  TextBlock = array[0..MaxText - 1] of Char;

var
  Data: ^TextBlock;
  GapStart: LongInt;     { first free cell: Data^[0..GapStart-1] is the text before the gap }
  GapEnd: LongInt;       { first cell after the gap: Data^[GapEnd..MaxText-1] is the text after }
  CursorPos: LongInt;
  WantedColumn: LongInt; { the column a run of vertical moves aims for }
  KeepColumn: Boolean;   { True while the last move was vertical, so WantedColumn is current }

function TextLength: LongInt;
begin
  TextLength := MaxText - (GapEnd - GapStart);
end;

function Cursor: LongInt;
begin
  Cursor := CursorPos;
end;

procedure Clear;
begin
  GapStart := 0;
  GapEnd := MaxText;
  CursorPos := 0;
  KeepColumn := False;
end;

{ The character at Pos, or #0 outside the text. Cells at or after the gap are shifted by the
  gap's size. }
function CharAt(Pos: LongInt): Char;
begin
  if (Pos < 0) or (Pos >= TextLength) then
    CharAt := #0
  else if Pos < GapStart then
    CharAt := Data^[Pos]
  else
    CharAt := Data^[Pos + (GapEnd - GapStart)];
end;

{ Moves the gap so that it starts at Pos, copying one character across it at a time. }
procedure MoveGapTo(Pos: LongInt);
begin
  while GapStart > Pos do
  begin
    Dec(GapStart);
    Dec(GapEnd);
    Data^[GapEnd] := Data^[GapStart];
  end;
  while GapStart < Pos do
  begin
    Data^[GapStart] := Data^[GapEnd];
    Inc(GapStart);
    Inc(GapEnd);
  end;
end;

{ The whole text as one block, for the compiler: with the gap at the end the text is
  Data^[0..TextLength-1]. The pointer is valid until the next edit. }
function TextPointer: Pointer;
begin
  MoveGapTo(TextLength);
  TextPointer := Data;
end;

{ ---- Lines and columns ---- }

function Clamp(Pos: LongInt): LongInt;
begin
  if Pos < 0 then
    Clamp := 0
  else if Pos > TextLength then
    Clamp := TextLength
  else
    Clamp := Pos;
end;

function LineStartPos(Pos: LongInt): LongInt;
begin
  Pos := Clamp(Pos);
  while (Pos > 0) and (CharAt(Pos - 1) <> LineBreak) do
    Dec(Pos);
  LineStartPos := Pos;
end;

{ The position of the line break that ends the line holding Pos, or TextLength. }
function LineEndPos(Pos: LongInt): LongInt;
begin
  Pos := Clamp(Pos);
  while (Pos < TextLength) and (CharAt(Pos) <> LineBreak) do
    Inc(Pos);
  LineEndPos := Pos;
end;

function ColumnOf(Pos: LongInt): LongInt;
begin
  Pos := Clamp(Pos);
  ColumnOf := Pos - LineStartPos(Pos) + 1;
end;

function LineOf(Pos: LongInt): LongInt;
var
  Scan, Line: LongInt;
begin
  Line := 1;
  for Scan := 0 to Clamp(Pos) - 1 do
    if CharAt(Scan) = LineBreak then
      Inc(Line);
  LineOf := Line;
end;

function LineCount: LongInt;
begin
  LineCount := LineOf(TextLength);
end;

{ The position of Line and Column. A line past the last is the last; a column past the end of
  its line is the end of the line. }
function PosOf(Line, Column: LongInt): LongInt;
var
  Pos, LineEnd: LongInt;
begin
  Pos := 0;
  while (Line > 1) and (Pos < TextLength) do
  begin
    if CharAt(Pos) = LineBreak then
      Dec(Line);
    Inc(Pos);
  end;
  Pos := LineStartPos(Pos);
  LineEnd := LineEndPos(Pos);
  if Column < 1 then
    Column := 1;
  if Column - 1 > LineEnd - Pos then
    PosOf := LineEnd
  else
    PosOf := Pos + Column - 1;
end;

{ ---- Cursor ---- }

procedure SetCursor(Pos: LongInt);
begin
  CursorPos := Clamp(Pos);
  KeepColumn := False;
end;

procedure MoveLeft;
begin
  SetCursor(CursorPos - 1);
end;

procedure MoveRight;
begin
  SetCursor(CursorPos + 1);
end;

procedure MoveToLineStart;
begin
  SetCursor(LineStartPos(CursorPos));
end;

procedure MoveToLineEnd;
begin
  SetCursor(LineEndPos(CursorPos));
end;

{ The cursor goes to the line starting at LineStart, in WantedColumn or at the line's end if
  that line is shorter. WantedColumn is remembered from the first move of a run, so moving
  through a short line comes back to the original column afterwards. }
procedure GoToLine(LineStart: LongInt);
var
  Target: LongInt;
begin
  if not KeepColumn then
  begin
    WantedColumn := ColumnOf(CursorPos);
    KeepColumn := True;
  end;
  Target := LineStart + WantedColumn - 1;
  if Target > LineEndPos(LineStart) then
    Target := LineEndPos(LineStart);
  CursorPos := Target;
end;

procedure MoveUp;
var
  Start: LongInt;
begin
  Start := LineStartPos(CursorPos);
  if Start > 0 then
    GoToLine(LineStartPos(Start - 1));
end;

procedure MoveDown;
var
  Finish: LongInt;
begin
  Finish := LineEndPos(CursorPos);
  if Finish < TextLength then
    GoToLine(Finish + 1);
end;

{ ---- Editing: the gap is moved to the cursor first ---- }

function IsFull: Boolean;
begin
  IsFull := TextLength = MaxText;
end;

{ Inserts Ch at the cursor; nothing happens when the text is full. }
procedure InsertChar(Ch: Char);
begin
  if not IsFull then
  begin
    MoveGapTo(CursorPos);
    Data^[GapStart] := Ch;
    Inc(GapStart);
    SetCursor(CursorPos + 1);
  end;
end;

{ Backspace: removes the character before the cursor by letting the gap grow to the left. }
procedure DeleteBefore;
begin
  if CursorPos > 0 then
  begin
    MoveGapTo(CursorPos);
    Dec(GapStart);
    SetCursor(CursorPos - 1);
  end;
end;

{ Delete: removes the character at the cursor by letting the gap grow to the right. }
procedure DeleteAfter;
begin
  if CursorPos < TextLength then
  begin
    MoveGapTo(CursorPos);
    Inc(GapEnd);
    KeepColumn := False;
  end;
end;

{ ---- Files ---- }

{ Replaces the text with the lines of the text file Name. Returns 0, the IOResult of opening
  it (the text is then unchanged), or LoadTooLarge (the text is then empty). Every line,
  the last included, ends with a LineBreak in the buffer. }
function Load(Name: string): Integer;
var
  F: Text;
  Ch: Char;
  Result: Integer;
begin
  {$I-}
  Assign(F, Name);
  Reset(F);
  Result := IOResult;
  if Result = 0 then
  begin
    Clear;
    while (Result = 0) and not Eof(F) do
    begin
      while (Result = 0) and not Eoln(F) do
      begin
        Read(F, Ch);
        if IsFull then
          Result := LoadTooLarge
        else
          InsertChar(Ch);
      end;
      ReadLn(F);
      if Result = 0 then
        if IsFull then
          Result := LoadTooLarge
        else
          InsertChar(LineBreak);
    end;
    Close(F);
    if Result <> 0 then
      Clear
    else
      SetCursor(0);
  end;
  {$I+}
  Load := Result;
end;

{ Writes the text to the text file Name, one line per LineBreak. Returns 0 or an IOResult. }
function Save(Name: string): Integer;
var
  F: Text;
  Pos: LongInt;
  Result: Integer;
begin
  {$I-}
  Assign(F, Name);
  Rewrite(F);
  Result := IOResult;
  if Result = 0 then
  begin
    for Pos := 0 to TextLength - 1 do
      if CharAt(Pos) = LineBreak then
        WriteLn(F)
      else
        Write(F, CharAt(Pos));
    Close(F);
    Result := IOResult;
  end;
  {$I+}
  Save := Result;
end;

begin
  GetMem(Data, MaxText);
  Clear;
end.
