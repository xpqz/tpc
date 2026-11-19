{ View: paints the edit window from the Buffer.

  The screen has three bands: row 1 is the menu bar (placeholder text until the Menu unit
  exists), rows 2 to 24 are the edit area, row 25 is the status line.

  Scrolling: TopLine is the first line shown and LeftColumn the first screen column shown.
  Paint moves them as little as possible so that the cursor is inside the edit area, then
  redraws the whole window. A full repaint is 25 writes of at most 80 characters; it is
  simple and always correct, so nothing tracks what changed (decision 45).

  Tabs: the Buffer keeps a tab as the single character #9. The View shows it as spaces up to
  the next multiple of TabWidth, so screen columns and Buffer columns differ on a line with
  tabs. Scrolling and the cursor use screen columns; the status line shows the Buffer column. }
unit View;

interface

procedure Reset;
procedure SetMessage(Text: string);
procedure SetInsertMode(Insert: Boolean);
procedure Paint;

{ Reads a line on the status line: Enter accepts (True), Esc cancels (False), Backspace edits;
  extended keys are ignored; at most MaxPromptText characters. Paints itself while it waits and
  clears the message at the end. }
function ReadPrompt(Prompt: string; var Text: string): Boolean;

implementation

uses Crt, Buffer;

const
  ScreenWidth = 80;
  MenuRow = 1;
  FirstEditRow = 2;
  EditRows = 23;
  StatusRow = 25;
  TabWidth = 8;
  BarAttr = $70;        { black on light grey }
  EditAttr = $1E;       { yellow on blue }
  MenuText = ' File  Edit  Run  Compile  Options';
  InsertTextColumn = 14;
  MessageColumn = 25;
  MaxPromptText = 40;
  BackspaceKey = #8;
  EnterKey = #13;
  EscKey = #27;

var
  TopLine: LongInt;
  LeftColumn: LongInt;
  Message: string;
  InsertMode: Boolean;

{ Clears the whole screen to the bar colour (so the one cell of row 25 that is never written,
  column 80, matches the status line) and forgets scrolling and the message. }
procedure Reset;
begin
  TopLine := 1;
  LeftColumn := 1;
  Message := '';
  InsertMode := True;
  TextAttr := BarAttr;
  ClrScr;
end;

procedure SetMessage(Text: string);
begin
  Message := Text;
end;

procedure SetInsertMode(Insert: Boolean);
begin
  InsertMode := Insert;
end;

{ How many screen columns the character at a screen column takes: a tab fills up to the
  next tab stop, anything else is one column. }
function CharWidth(Ch: Char; Column: LongInt): LongInt;
begin
  if Ch = #9 then
    CharWidth := TabWidth - (Column - 1) mod TabWidth
  else
    CharWidth := 1;
end;

{ The screen column (1-based, before scrolling) of Pos on its line. }
function ScreenColumnOf(Pos: LongInt): LongInt;
var
  Scan, Column: LongInt;
begin
  Column := 1;
  for Scan := LineStartPos(Pos) to Pos - 1 do
    Inc(Column, CharWidth(Buffer.CharAt(Scan), Column));
  ScreenColumnOf := Column;
end;

procedure ScrollToCursor;
var
  Line, Column: LongInt;
begin
  Line := LineOf(Cursor);
  if Line < TopLine then
    TopLine := Line
  else if Line > TopLine + EditRows - 1 then
    TopLine := Line - EditRows + 1;
  Column := ScreenColumnOf(Cursor);
  if Column < LeftColumn then
    LeftColumn := Column
  else if Column > LeftColumn + ScreenWidth - 1 then
    LeftColumn := Column - ScreenWidth + 1;
end;

function Padded(Text: string; Width: Integer): string;
begin
  while Length(Text) < Width do
    Text := Text + ' ';
  Padded := Copy(Text, 1, Width);
end;

{ The ScreenWidth characters of text line Line that are inside the window: tabs expanded,
  everything left of LeftColumn and right of the window cut off, the rest blank. }
function VisibleCells(Line: LongInt): string;
var
  Cells: string;
  Pos, LineEnd, Column, Width, Used: LongInt;
begin
  Cells := '';
  if Line <= LineCount then
  begin
    Pos := PosOf(Line, 1);
    LineEnd := LineEndPos(Pos);
    Column := 1;
    while (Pos < LineEnd) and (Column < LeftColumn + ScreenWidth) do
    begin
      Width := CharWidth(Buffer.CharAt(Pos), Column);
      for Used := 1 to Width do
      begin
        if Column >= LeftColumn then
          if Buffer.CharAt(Pos) = #9 then
            Cells := Cells + ' '
          else
            Cells := Cells + Buffer.CharAt(Pos);
        Inc(Column);
      end;
      Inc(Pos);
    end;
  end;
  VisibleCells := Padded(Cells, ScreenWidth);
end;

function StatusText: string;
var
  Position, Line, Indicator: string;
begin
  Str(LineOf(Cursor), Line);
  Str(ColumnOf(Cursor), Position);
  if InsertMode then
    Indicator := 'Insert'
  else
    Indicator := 'Overwrite';
  Line := Padded(' ' + Line + ':' + Position, InsertTextColumn - 1);
  StatusText := Padded(Line + Indicator, MessageColumn - 1) + Message;
end;

{ Row 25 is written only up to column 79: a character written at column 80 of the last row
  scrolls the screen. }
procedure Paint;
var
  Row: Integer;
begin
  ScrollToCursor;
  TextAttr := BarAttr;
  GotoXY(1, MenuRow);
  Write(Padded(MenuText, ScreenWidth));
  TextAttr := EditAttr;
  for Row := 0 to EditRows - 1 do
  begin
    GotoXY(1, FirstEditRow + Row);
    Write(VisibleCells(TopLine + Row));
  end;
  TextAttr := BarAttr;
  GotoXY(1, StatusRow);
  Write(Padded(StatusText, ScreenWidth - 1));
  GotoXY(ScreenColumnOf(Cursor) - LeftColumn + 1, LineOf(Cursor) - TopLine + FirstEditRow);
end;

function ReadPrompt(Prompt: string; var Text: string): Boolean;
var
  Key: Char;
  Finished, Accepted: Boolean;
begin
  Finished := False;
  Accepted := False;
  Text := '';
  while not Finished do
  begin
    SetMessage(Prompt + Text);
    Paint;
    Key := ReadKey;
    if Key = EnterKey then
    begin
      Finished := True;
      Accepted := True;
    end
    else if Key = EscKey then
      Finished := True
    else if Key = #0 then
      Key := ReadKey
    else if Key = BackspaceKey then
    begin
      if Length(Text) > 0 then
        Text := Copy(Text, 1, Length(Text) - 1);
    end
    else if (Key >= ' ') and (Length(Text) < MaxPromptText) then
      Text := Text + Key;
  end;
  SetMessage('');
  ReadPrompt := Accepted;
end;

end.
