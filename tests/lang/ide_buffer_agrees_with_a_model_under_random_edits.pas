{ Buffer: 3000 pseudo-random operations are applied to the buffer and to a plain array that
  does the same naively; the two must agree on length, cursor, line and column throughout,
  and on every character at the end. The printed numbers (final length, line count,
  checksum) also come out of TP 5.0. }
program AgreesWithAModelUnderRandomEdits;

uses Buffer;

const
  Limit = 600;        { the model's size; inserts beyond it are skipped in both }

var
  Model: array[0..Limit] of Char;
  ModelLen, ModelCur: Integer;
  Seed: LongInt;
  Mismatches, Op, I: Integer;
  Line, Col: Integer;
  Sum: LongInt;
  Ch: Char;

function Pick(Range: Integer): Integer;
begin
  Seed := (Seed * 25173 + 13849) mod 65536;
  Pick := (Seed shr 4) mod Range;
end;

procedure ModelInsert(C: Char);
var
  I: Integer;
begin
  for I := ModelLen downto ModelCur + 1 do
    Model[I] := Model[I - 1];
  Model[ModelCur] := C;
  Inc(ModelLen);
  Inc(ModelCur);
end;

procedure ModelDeleteAt(P: Integer);
var
  I: Integer;
begin
  for I := P to ModelLen - 2 do
    Model[I] := Model[I + 1];
  Dec(ModelLen);
end;

procedure ModelLineAndColumn(var L, C: Integer);
var
  I: Integer;
begin
  L := 1;
  C := 1;
  for I := 0 to ModelCur - 1 do
    if Model[I] = LineBreak then
    begin
      Inc(L);
      C := 1;
    end
    else
      Inc(C);
end;

procedure Check(What: string);
begin
  ModelLineAndColumn(Line, Col);
  if (TextLength <> ModelLen) or (Cursor <> ModelCur) or (LineOf(Cursor) <> Line)
    or (ColumnOf(Cursor) <> Col) then
  begin
    Inc(Mismatches);
    if Mismatches < 5 then
      WriteLn('mismatch after ', What, ': ', TextLength, '/', ModelLen, ' ', Cursor, '/',
        ModelCur, ' ', LineOf(Cursor), ':', ColumnOf(Cursor), '/', Line, ':', Col);
  end;
end;

begin
  Seed := 12345;
  ModelLen := 0;
  ModelCur := 0;
  Mismatches := 0;
  for Op := 1 to 3000 do
    case Pick(10) of
      0, 1, 2, 3:
        if ModelLen < Limit then
        begin
          if Pick(6) = 0 then Ch := LineBreak else Ch := Chr(97 + Pick(26));
          InsertChar(Ch);
          ModelInsert(Ch);
          Check('insert');
        end;
      4:
        begin
          DeleteBefore;
          if ModelCur > 0 then
          begin
            ModelDeleteAt(ModelCur - 1);
            Dec(ModelCur);
          end;
          Check('backspace');
        end;
      5:
        begin
          DeleteAfter;
          if ModelCur < ModelLen then
            ModelDeleteAt(ModelCur);
          Check('delete');
        end;
      6:
        begin
          MoveLeft;
          if ModelCur > 0 then Dec(ModelCur);
          Check('left');
        end;
      7:
        begin
          MoveRight;
          if ModelCur < ModelLen then Inc(ModelCur);
          Check('right');
        end;
      8:
        begin
          I := Pick(ModelLen + 1);
          SetCursor(I);
          ModelCur := I;
          Check('set cursor');
        end;
      9:
        begin
          MoveToLineStart;
          while (ModelCur > 0) and (Model[ModelCur - 1] <> LineBreak) do Dec(ModelCur);
          Check('line start');
          if Pick(2) = 0 then
          begin
            MoveToLineEnd;
            while (ModelCur < ModelLen) and (Model[ModelCur] <> LineBreak) do Inc(ModelCur);
            Check('line end');
          end;
        end;
    end;
  WriteLn('mismatches ', Mismatches);
  Sum := 0;
  Mismatches := 0;
  for I := 0 to ModelLen - 1 do
  begin
    if CharAt(I) <> Model[I] then Inc(Mismatches);
    Sum := (Sum * 31 + Ord(Model[I])) mod 1000003;
  end;
  WriteLn('character mismatches ', Mismatches);
  WriteLn('length ', TextLength, ' cursor ', Cursor, ' lines ', LineCount, ' checksum ', Sum);
end.
