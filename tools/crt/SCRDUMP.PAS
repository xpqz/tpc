{ Dumps the final 80x25 text screen to SCREEN.TXT when the program ends. It is the first unit
  of the program's uses clause, so its exit procedure runs last. Format (docs/abi.md, the
  screen dump): 25 rows of text with trailing blanks cut, then
    cursor COL ROW          1-based, from the BIOS data area
    exit CODE               ExitCode
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

function CellAttr(Row, Col: Integer): Byte;
begin
  CellAttr := Mem[$B800: Row * 160 + Col * 2 + 1];
end;

procedure DumpText(var T: Text);
var
  Row, Col, Last: Integer;
  Line: string;
begin
  for Row := 0 to 24 do
  begin
    Line := '';
    for Col := 0 to 79 do
      Line := Line + Chr(Mem[$B800: Row * 160 + Col * 2]);
    Last := 80;
    while (Last > 0) and (Line[Last] = ' ') do Dec(Last);
    WriteLn(T, Copy(Line, 1, Last));
  end;
end;

procedure DumpAttributes(var T: Text);
var
  Row, Col, RunStart: Integer;
  Attr: Byte;
begin
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
end;

{$F+}
procedure Dump;
var
  T: Text;
begin
  ExitProc := OldExit;
  Assign(T, 'SCREEN.TXT');
  Rewrite(T);
  DumpText(T);
  WriteLn(T, 'cursor ', Mem[$40:$50] + 1, ' ', Mem[$40:$51] + 1);
  WriteLn(T, 'exit ', ExitCode);
  DumpAttributes(T);
  Close(T);
end;
{$F-}

begin
  OldExit := ExitProc;
  ExitProc := @Dump;
end.
