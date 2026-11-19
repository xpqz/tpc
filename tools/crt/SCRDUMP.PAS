{ Prototype: dumps the final 80x25 text screen to SCREEN.TXT when the program ends. Meant to
  be the first unit of the program's uses clause, so its exit procedure runs last.
  Format: 25 rows of text (trailing blanks cut), then
    cursor COL ROW          1-based, from the BIOS data area
    exit CODE               ExitCode, with `error SEG:OFS` when ErrorAddr is set
    attr ROW COL LEN HEX    one line per run of cells whose attribute is not $07 }
unit ScrDump;
interface
implementation
var
  OldExit: Pointer;

function Hex2(B: Byte): string;
const
  Digits: string = '0123456789ABCDEF';
begin
  Hex2 := Digits[B shr 4 + 1] + Digits[B and 15 + 1];
end;

function Num(N: Integer): string;
var
  S: string;
begin
  Str(N, S);
  Num := S;
end;

{$F+}
procedure Dump;
var
  Row, Col, Last, RunStart: Integer;
  Line: string;
  T: Text;
  Attr: Byte;

  function CellAttr(R, C: Integer): Byte;
  begin
    CellAttr := Mem[$B800: R * 160 + C * 2 + 1];
  end;

begin
  ExitProc := OldExit;
  Assign(T, 'SCREEN.TXT');
  Rewrite(T);
  for Row := 0 to 24 do
  begin
    Line := '';
    for Col := 0 to 79 do
      Line := Line + Chr(Mem[$B800: Row * 160 + Col * 2]);
    Last := 80;
    while (Last > 0) and (Line[Last] = ' ') do Dec(Last);
    WriteLn(T, Copy(Line, 1, Last));
  end;
  WriteLn(T, 'cursor ', Mem[$40:$50] + 1, ' ', Mem[$40:$51] + 1);
  Write(T, 'exit ', ExitCode);
  if ErrorAddr <> nil then
    Write(T, ' error ', Hex2(Seg(ErrorAddr^) shr 8), Hex2(Seg(ErrorAddr^)), ':',
      Hex2(Ofs(ErrorAddr^) shr 8), Hex2(Ofs(ErrorAddr^)));
  WriteLn(T);
  for Row := 0 to 24 do
  begin
    Col := 0;
    while Col < 80 do
    begin
      Attr := CellAttr(Row, Col);
      RunStart := Col;
      while (Col < 80) and (CellAttr(Row, Col) = Attr) do Inc(Col);
      if Attr <> 7 then
        WriteLn(T, 'attr ', Row + 1, ' ', RunStart + 1, ' ', Col - RunStart, ' ', Hex2(Attr));
    end;
  end;
  Close(T);
end;
{$F-}

begin
  OldExit := ExitProc;
  ExitProc := @Dump;
end.
