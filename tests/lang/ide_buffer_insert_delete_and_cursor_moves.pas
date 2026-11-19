{ Buffer: typing, Backspace, Delete, cursor moves by character and the contiguous text. }
program InsertDeleteAndCursorMoves;

uses Buffer;

type
  Block = array[0..65520] of Char;

procedure Show(Title: string);
var
  Pos: LongInt;
begin
  Write(Title, ': len ', TextLength, ' cur ', Cursor, ' [');
  for Pos := 0 to TextLength - 1 do
    if CharAt(Pos) = LineBreak then Write('|') else Write(CharAt(Pos));
  WriteLn(']');
end;

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

var
  P: ^Block;
  I: LongInt;
  Sum: LongInt;
begin
  Show('empty');
  WriteLn('full: ', IsFull);
  DeleteBefore;
  DeleteAfter;
  Show('still empty');
  WriteLn('char at 0 of empty is #', Ord(CharAt(0)));

  Type_('hello');
  Show('typed');
  MoveLeft; MoveLeft;
  Show('left 2');
  InsertChar('X');
  Show('inserted');
  DeleteBefore;
  Show('backspaced');
  DeleteAfter;
  Show('deleted');
  SetCursor(0);
  DeleteBefore;
  Show('at start');
  MoveLeft;
  Show('left at start stays');
  InsertChar('>');
  Show('inserted at start');
  SetCursor(TextLength);
  MoveRight;
  Show('right at end stays');
  DeleteAfter;
  Show('end');

  SetCursor(-5);
  Show('set cursor below 0');
  SetCursor(1000);
  Show('set cursor past end');
  SetCursor(3);
  Show('set cursor 3');
  WriteLn('char at -1 is #', Ord(CharAt(-1)), ', at len is #', Ord(CharAt(TextLength)));
  WriteLn('chars around the cursor: ', CharAt(2), CharAt(3));

  { The text is contiguous in the block after TextPointer, wherever the gap was. }
  P := TextPointer;
  Sum := 0;
  for I := 0 to TextLength - 1 do
    Sum := Sum * 3 + Ord(P^[I]) mod 1000;
  WriteLn('first ', P^[0], ' second ', P^[1], ' sum ', Sum);
  Show('after TextPointer');
  InsertChar('!');
  Show('edit after TextPointer');

  { Edits at several places leave the same text a straightforward string would. }
  Clear;
  Show('cleared');
  Type_('abc/def');
  SetCursor(2); DeleteAfter; DeleteAfter; DeleteAfter;
  Show('deleted across a break');
  InsertChar('Z');
  SetCursor(0);
  Type_('12/');
  Show('typed at start');
  for I := 1 to 20 do
    DeleteBefore;
  Show('backspaced past the start');
end.
