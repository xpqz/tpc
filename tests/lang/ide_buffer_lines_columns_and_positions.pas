{ Buffer: line count, line and column of a position, position of a line and column, line
  start and end, and vertical moves that keep the wanted column. }
program LinesColumnsAndPositions;

uses Buffer;

procedure Type_(S: string);
var
  I: Integer;
begin
  for I := 1 to Length(S) do
    if S[I] = '/' then
      InsertChar(LineBreak)
    else
      InsertChar(S[I]);
end;

procedure Where(Title: string);
begin
  WriteLn(Title, ': pos ', Cursor, ' line ', LineOf(Cursor), ' col ', ColumnOf(Cursor));
end;

procedure Table;
var
  Pos: LongInt;
begin
  for Pos := -1 to TextLength + 1 do
    WriteLn('pos ', Pos, ': line ', LineOf(Pos), ' col ', ColumnOf(Pos), ' start ',
      LineStartPos(Pos), ' end ', LineEndPos(Pos));
end;

var
  Line, Column: LongInt;
begin
  WriteLn('empty: lines ', LineCount, ' pos(1,1) ', PosOf(1, 1), ' pos(5,9) ', PosOf(5, 9));
  Type_('one/two words/ /last');
  WriteLn('lines ', LineCount, ' length ', TextLength);
  Table;
  for Line := 0 to 6 do
    for Column := 0 to 12 do
      if Column mod 6 = 0 then
        WriteLn('posof ', Line, ',', Column, ' = ', PosOf(Line, Column));

  { Vertical moves from the end of the long line through the empty line. }
  SetCursor(PosOf(2, 10));
  Where('start on 2:10');
  MoveDown; Where('down to the blank line');
  MoveDown; Where('down to last');
  MoveDown; Where('down at the last line');
  MoveUp; Where('up to blank');
  MoveUp; Where('up to line 2');
  MoveUp; Where('up to line 1');
  MoveUp; Where('up at the first line');
  MoveToLineEnd; Where('line end');
  MoveDown; Where('down keeps the end column');
  MoveToLineStart; Where('line start');
  MoveUp; Where('up from column 1');
  MoveRight; MoveRight; Where('right 2');
  MoveDown; Where('down');
  MoveLeft; Where('left resets the wanted column');
  MoveDown; Where('down');
  MoveRight; MoveRight; MoveRight; Where('right across a break');
  MoveToLineEnd; Where('end of last line');
  MoveToLineEnd; Where('end again');

  { Typing sets the wanted column, a Delete does too. }
  SetCursor(PosOf(2, 9));
  InsertChar('!');
  MoveUp; Where('up after typing in column 9');
  SetCursor(PosOf(2, 9));
  DeleteBefore;
  MoveUp; Where('up after backspace');

  { Every edit and every horizontal move forgets the wanted column; only vertical moves keep it. }
  Clear;
  Type_('abcdef/xy/abcdef');
  SetCursor(PosOf(1, 6));
  MoveDown; Where('down to the short line');
  MoveDown; Where('down keeps column 6');
  SetCursor(PosOf(1, 6));
  MoveDown; InsertChar('Q'); MoveDown; Where('down after typing');
  SetCursor(PosOf(1, 6));
  MoveDown; DeleteBefore; MoveDown; Where('down after backspace');
  SetCursor(PosOf(1, 6));
  MoveDown; SetCursor(PosOf(2, 1)); DeleteAfter; MoveDown; Where('down after delete');

  { A text ending in a break has an empty last line. }
  Clear;
  Type_('a/b/');
  WriteLn('trailing break: lines ', LineCount, ' last line starts at ', PosOf(3, 1),
    ' line of end ', LineOf(TextLength), ' col ', ColumnOf(TextLength));
  SetCursor(0);
  MoveToLineEnd; MoveDown; MoveDown; Where('down to the empty last line');
  MoveDown; Where('and no further');
  { Only breaks. }
  Clear;
  Type_('///');
  WriteLn('four empty lines: ', LineCount, ' pos(3,5) ', PosOf(3, 5), ' pos(9,1) ', PosOf(9, 1));
end.
