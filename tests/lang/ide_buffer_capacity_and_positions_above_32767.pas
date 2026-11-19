{ Buffer: a full 65521-character text, positions and indexes above 32767, and the gap crossing
  the whole block. Lines are 20 characters including the break. The checksums look at every
  7th character only: the oracle runs this under an emulated 8086, where a full pass is slow. }
program CapacityAndPositionsAbove32767;

uses Buffer;

type
  Block = array[0..65520] of Char;

function Checksum: LongInt;
var
  Pos, Sum: LongInt;
begin
  Sum := 0;
  Pos := 0;
  while Pos < TextLength do
  begin
    Sum := Sum + Ord(CharAt(Pos)) * (Pos mod 251 + 1);
    Pos := Pos + 7;
  end;
  Checksum := Sum;
end;

function BlockChecksum: LongInt;
var
  P: ^Block;
  Pos, Sum: LongInt;
begin
  P := TextPointer;
  Sum := 0;
  Pos := 0;
  while Pos < TextLength do
  begin
    Sum := Sum + Ord(P^[Pos]) * (Pos mod 251 + 1);
    Pos := Pos + 7;
  end;
  BlockChecksum := Sum;
end;

procedure Where(Title: string);
begin
  WriteLn(Title, ': pos ', Cursor, ' col ', ColumnOf(Cursor));
end;

var
  I, Before: LongInt;
  Ch: Char;
begin
  { Five more characters than fit: the last five are refused. }
  for I := 0 to MaxText + 4 do
  begin
    if I mod 20 = 19 then Ch := LineBreak else Ch := Chr(65 + I mod 26);
    InsertChar(Ch);
  end;
  WriteLn('length ', TextLength, ' cursor ', Cursor, ' full ', IsFull);
  InsertChar('x');
  WriteLn('insert when full: length ', TextLength, ' cursor ', Cursor);
  WriteLn('lines ', LineCount, ' checksum ', Checksum);
  WriteLn('line of 40000: ', LineOf(40000), ' col ', ColumnOf(40000), ' posof(2001,1) ',
    PosOf(2001, 1), ' posof(2001,21) ', PosOf(2001, 21), ' posof(3277,5) ', PosOf(3277, 5));
  WriteLn('last line starts ', LineStartPos(65520), ' ends ', LineEndPos(65520));
  WriteLn('chars: ', CharAt(32766), CharAt(32767), CharAt(32768), CharAt(65520),
    ' past end #', Ord(CharAt(65521)));

  SetCursor(50000);
  Where('cursor 50000');
  WriteLn('line of the cursor ', LineOf(Cursor));
  MoveUp; Where('up');
  MoveDown; MoveDown; Where('down 2');
  MoveToLineEnd; Where('line end');
  MoveRight; Where('right');
  SetCursor(32767); MoveRight; MoveRight; Where('across 32767');
  WriteLn('line of the cursor ', LineOf(Cursor));
  MoveLeft; MoveLeft; MoveLeft; Where('back across');

  { Delete at one end of the block, type at the other, with the full text in between. }
  SetCursor(65000);
  for I := 1 to 100 do
    DeleteBefore;
  WriteLn('deleted 100: length ', TextLength, ' cursor ', Cursor, ' full ', IsFull);
  SetCursor(10);
  Before := TextLength;
  for I := 1 to 150 do
    InsertChar('#');
  WriteLn('typed 150: length ', TextLength, ' accepted ', TextLength - Before, ' cursor ',
    Cursor, ' full ', IsFull);
  WriteLn('checksum ', Checksum);
  SetCursor(MaxText);
  Where('end');
  WriteLn('line of the end ', LineOf(Cursor));
  for I := 1 to 3000 do
    DeleteBefore;
  WriteLn('after deleting 3000: length ', TextLength, ' cursor ', Cursor);
  SetCursor(0);
  for I := 1 to 3000 do
    DeleteAfter;
  WriteLn('after deleting 3000 more: length ', TextLength, ' lines ', LineCount);
  Before := BlockChecksum;
  WriteLn('block checksum ', Before, ' same ', Before = Checksum);
  Clear;
  WriteLn('cleared: length ', TextLength);
end.
